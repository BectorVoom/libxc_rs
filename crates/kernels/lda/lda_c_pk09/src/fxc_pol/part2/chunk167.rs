//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 167/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk167<F: Float>(t429: F, t545: F, t337: F, t533: F, t430: F, t132: F, t92: F, t142: F, t444: F, t455: F, t476: F, t478: F, t483: F, t485: F, t490: F, t498: F, t502: F, t508: F, t516: F, t525: F, t538: F) -> (F, F, F, F, F, F) {
    let t546 = t545 * t429;
    let t549 = t533 * t337;
    let t550 = t549 * t430;
    let t551 = t92 * t132;
    let t552 = t142 * t551;
    let t555 = t444 * t485 - F::new(22.07984838129906) * t478 - F::new(2.700122570973095) * t483 - F::new(3.7610742193750633) * t490 * t455 + F::new(1.8805371096875316) * t498 * t455 + F::new(19.489173774580152) * t502 * t455 + F::new(4.937333717448355) * t508 * t455 - F::new(0.04115066352984959) * t476 * t516 + F::new(18.635258017632964) * t525 * t455 - F::new(2.2140749178833072) * t538 * t455 - F::new(2.427516195194328) * t546 * t455 - F::new(3.5540878740919255) * t550 * t552;
    (t546, t549, t550, t551, t552, t555)
}
