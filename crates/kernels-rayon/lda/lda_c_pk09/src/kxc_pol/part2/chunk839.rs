//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 839/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk839(t192: f64, t2214: f64, t2314: f64, t3753: f64, t4411: f64, t4660: f64, t709: f64, t713: f64, t7706: f64, t7727: f64, t7776: f64, t8555: f64, t8560: f64, t8564: f64, t8566: f64, t8571: f64, t8573: f64) -> f64 {
    let t8575 = 2.427516195194328_f64 * t3753 * t2214 + 2.2140749178833072_f64 * t192 * t7776 + 2.2140749178833072_f64 * t192 * t7706 - 1.8805371096875316_f64 * t8555 * t713 - 1.8805371096875316_f64 * t8555 * t709 + 19.489173774580152_f64 * t8560 + 2.2140749178833072_f64 * t7727 * t713 + 12.992782516386768_f64 * t8564 + 1.2536914064583544_f64 * t8566 - t4660 * t2314 + 14.71989892086604_f64 * t4411 + 2.2140749178833072_f64 * t8571 - 3.2915558116322368_f64 * t8573;
    t8575
}
