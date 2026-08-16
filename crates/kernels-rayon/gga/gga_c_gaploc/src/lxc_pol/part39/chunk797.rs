//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 797/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk797(t12670: f64, t2610: f64, t3720: f64, t2365: f64, t2033: f64, t12252: f64, t959: f64, t12693: f64, t12706: f64, t13861: f64, t1457: f64, t2103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13890 = 0.38342925953920749677e0_f64 * t12670;
    let t13891 = t2610 * t3720;
    let t13892 = t2365 * t13891;
    let t13893 = t2033 * t13892;
    let t13895 = t12252 * t959;
    let t13898 = 0.63904876589867916128e-1_f64 * t12693;
    let t13899 = 0.63904876589867916128e-1_f64 * t12706;
    let t13900 = t1457 * t13861;
    let t13901 = t2103 * t13900;
    (t13890, t13891, t13892, t13893, t13895, t13898, t13899, t13900, t13901)
}
