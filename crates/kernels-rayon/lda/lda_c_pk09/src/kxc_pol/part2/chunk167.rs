//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 167/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk167(t429: f64, t545: f64, t337: f64, t533: f64, t430: f64, t132: f64, t92: f64, t142: f64, t444: f64, t455: f64, t476: f64, t478: f64, t483: f64, t485: f64, t490: f64, t498: f64, t502: f64, t508: f64, t516: f64, t525: f64, t538: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t546 = t545 * t429;
    let t549 = t533 * t337;
    let t550 = t549 * t430;
    let t551 = t92 * t132;
    let t552 = t142 * t551;
    let t555 = t444 * t485 - 22.07984838129906_f64 * t478 - 2.700122570973095_f64 * t483 - 3.7610742193750633_f64 * t490 * t455 + 1.8805371096875316_f64 * t498 * t455 + 19.489173774580152_f64 * t502 * t455 + 4.937333717448355_f64 * t508 * t455 - 0.04115066352984959_f64 * t476 * t516 + 18.635258017632964_f64 * t525 * t455 - 2.2140749178833072_f64 * t538 * t455 - 2.427516195194328_f64 * t546 * t455 - 3.5540878740919255_f64 * t550 * t552;
    (t546, t549, t550, t551, t552, t555)
}
