//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 982/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk982<F: Float>(t10489: F, t10501: F, t1363: F, t310: F, t1435: F, t2690: F, t10025: F, t10435: F, t10441: F, t10443: F, t10447: F, t10450: F, t10455: F, t10459: F, t10466: F, t10468: F, t10471: F, t10475: F, t1290: F, t1304: F, t1348: F, t1629: F, t2587: F, t2629: F, t2667: F, t311: F, t5033: F, t5613: F, t6168: F, t93: F) -> F {
    let t10502 = t10489 + t10501;
    let t10503 = t10502 * t1363;
    let t10504 = t310 * t10503;
    let t10507 = t2690 * t1435;
    let t10510 = F::cast_from(2.427516195194328_f64) * t2690 * t1629 + F::cast_from(0.8885219685229814_f64) * t6168 * t93 * t10435 + F::cast_from(3.5540878740919255_f64) * t10441 - F::cast_from(3.5540878740919255_f64) * t5613 * t93 * t10443 + F::cast_from(0.8091720650647759_f64) * t10447 - F::cast_from(19.489173774580152_f64) * t10450 - F::cast_from(19.489173774580152_f64) * t2629 * t1629 - F::cast_from(1.8805371096875316_f64) * t10455 - F::cast_from(1.8805371096875316_f64) * t2667 * t1629 + F::cast_from(19.489173774580152_f64) * t10459 + F::cast_from(38.978347549160304_f64) * t1290 * t10025 - F::cast_from(19.489173774580152_f64) * t1304 * t2587 - F::cast_from(1.8805371096875316_f64) * t10466 * t10468 - F::cast_from(2.427516195194328_f64) * t10471 * t311 + F::cast_from(2.427516195194328_f64) * t10475 - F::cast_from(0.04115066352984959_f64) * t1348 * t10504 + F::cast_from(0.8091720650647759_f64) * t10507 - F::cast_from(4.937333717448355_f64) * t5033;
    t10510
}
