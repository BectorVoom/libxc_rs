//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2432/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432(t11452: f64, t2962: f64, t41306: f64, t3335: f64, t1071: f64, t3043: f64, t12032: f64, t342: f64, t11902: f64, t378: f64, t3046: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41895 = t2962 * t11452;
    let t41908 = 0.17757530864197530864e0_f64 * t41306;
    let t41936 = t3335 * t3335;
    let t41937 = 1.0_f64 / t41936;
    let t41993 = t3043 * t1071;
    let t42013 = 0.86419753086419753087e-1_f64 * t41306;
    let t42038 = t342 * t12032;
    let t42041 = t11902 * t378;
    let t42044 = t3046 * t3259;
    (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044)
}
