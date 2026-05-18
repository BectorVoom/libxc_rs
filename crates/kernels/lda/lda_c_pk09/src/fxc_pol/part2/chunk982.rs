//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 982/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk982<F: Float>(t10489: F, t10501: F, t1363: F, t310: F, t1435: F, t2690: F, t10025: F, t10435: F, t10441: F, t10443: F, t10447: F, t10450: F, t10455: F, t10459: F, t10466: F, t10468: F, t10471: F, t10475: F, t1290: F, t1304: F, t1348: F, t1629: F, t2587: F, t2629: F, t2667: F, t311: F, t5033: F, t5613: F, t6168: F, t93: F) -> F {
    let t10502 = t10489 + t10501;
    let t10503 = t10502 * t1363;
    let t10504 = t310 * t10503;
    let t10507 = t2690 * t1435;
    let t10510 = F::new(2.427516195194328) * t2690 * t1629 + F::new(0.8885219685229814) * t6168 * t93 * t10435 + F::new(3.5540878740919255) * t10441 - F::new(3.5540878740919255) * t5613 * t93 * t10443 + F::new(0.8091720650647759) * t10447 - F::new(19.489173774580152) * t10450 - F::new(19.489173774580152) * t2629 * t1629 - F::new(1.8805371096875316) * t10455 - F::new(1.8805371096875316) * t2667 * t1629 + F::new(19.489173774580152) * t10459 + F::new(38.978347549160304) * t1290 * t10025 - F::new(19.489173774580152) * t1304 * t2587 - F::new(1.8805371096875316) * t10466 * t10468 - F::new(2.427516195194328) * t10471 * t311 + F::new(2.427516195194328) * t10475 - F::new(0.04115066352984959) * t1348 * t10504 + F::new(0.8091720650647759) * t10507 - F::new(4.937333717448355) * t5033;
    t10510
}
