//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1310/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1310(t14193: f64, t22493: f64, t53060: f64, t14185: f64, t3306: f64, t353: f64, t859: f64, t1105: f64, t4111: f64, t4386: f64, t1206: f64, t2494: f64) -> (f64, f64, f64, f64, f64) {
    let t54942 = 7.0_f64 / 144.0_f64 * t22493 * t14193;
    let t54946 = 7.0_f64 / 288.0_f64 * t53060;
    let t54952 = t859 * t353 * t14185 * t3306;
    let t54957 = t4386 * t353 * t4111 * t1105;
    let t54962 = t4386 * t353 * t1206 * t2494;
    (t54942, t54946, t54952, t54957, t54962)
}
