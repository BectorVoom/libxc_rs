//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1128/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1128(t2053: f64, t4058: f64, t1198: f64, t6854: f64, t1105: f64, t13751: f64, t944: f64, t2494: f64, t3944: f64, t4188: f64, t945: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14149 = t4058 * t2053;
    let t14153 = t1198 * t6854;
    let t14380 = t13751 * t1105;
    let t14383 = t1105 * t944;
    let t14387 = t3944 * t2494;
    let t14390 = t4188 * t945;
    let t14392 = t14390 * t810;
    (t14149, t14153, t14380, t14383, t14387, t14390, t14392)
}
