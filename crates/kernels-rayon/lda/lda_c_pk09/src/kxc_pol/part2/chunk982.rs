//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 982/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk982(t10489: f64, t10501: f64, t1363: f64, t310: f64, t1435: f64, t2690: f64, t10025: f64, t10435: f64, t10441: f64, t10443: f64, t10447: f64, t10450: f64, t10455: f64, t10459: f64, t10466: f64, t10468: f64, t10471: f64, t10475: f64, t1290: f64, t1304: f64, t1348: f64, t1629: f64, t2587: f64, t2629: f64, t2667: f64, t311: f64, t5033: f64, t5613: f64, t6168: f64, t93: f64) -> f64 {
    let t10502 = t10489 + t10501;
    let t10503 = t10502 * t1363;
    let t10504 = t310 * t10503;
    let t10507 = t2690 * t1435;
    let t10510 = 2.427516195194328_f64 * t2690 * t1629 + 0.8885219685229814_f64 * t6168 * t93 * t10435 + 3.5540878740919255_f64 * t10441 - 3.5540878740919255_f64 * t5613 * t93 * t10443 + 0.8091720650647759_f64 * t10447 - 19.489173774580152_f64 * t10450 - 19.489173774580152_f64 * t2629 * t1629 - 1.8805371096875316_f64 * t10455 - 1.8805371096875316_f64 * t2667 * t1629 + 19.489173774580152_f64 * t10459 + 38.978347549160304_f64 * t1290 * t10025 - 19.489173774580152_f64 * t1304 * t2587 - 1.8805371096875316_f64 * t10466 * t10468 - 2.427516195194328_f64 * t10471 * t311 + 2.427516195194328_f64 * t10475 - 0.04115066352984959_f64 * t1348 * t10504 + 0.8091720650647759_f64 * t10507 - 4.937333717448355_f64 * t5033;
    t10510
}
