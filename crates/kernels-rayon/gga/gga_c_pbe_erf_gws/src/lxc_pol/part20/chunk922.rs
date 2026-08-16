//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 922/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk922(t5103: f64, t2660: f64, t3451: f64, t1879: f64, t4358: f64, t532: f64, t198: f64, t186: f64, t561: f64, t2737: f64, t2741: f64, t2730: f64, t3564: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10307 = 4.0_f64 / 135.0_f64 * t5103;
    let t10309 = 4.0_f64 / 15.0_f64 * t2660 * t3451;
    let t10311 = 4.0_f64 / 15.0_f64 * t1879 * t3451;
    let t10313 = -t532 - 3.0_f64 * t4358;
    let t10314 = t198 * t10313;
    let t10315 = t186 * t10314;
    let t10317 = 4.0_f64 / 15.0_f64 * t561 * t10315;
    let t10319 = 8.0_f64 / 15.0_f64 * t2741 * t2737;
    let t10321 = 4.0_f64 / 15.0_f64 * t2730 * t3564;
    (t10307, t10309, t10311, t10313, t10317, t10319, t10321)
}
