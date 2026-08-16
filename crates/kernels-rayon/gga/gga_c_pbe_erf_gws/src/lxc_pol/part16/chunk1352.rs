//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1352/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1352(t54491: f64, t14954: f64, t4414: f64, t14981: f64, t15004: f64, t840: f64, t54504: f64, t1105: f64, t12213: f64, t14200: f64, t14240: f64, t14272: f64, t15081: f64, t2376: f64, t2408: f64, t2409: f64, t2494: f64, t3066: f64, t3067: f64, t3306: f64, t4110: f64, t54496: f64, t54502: f64, t54508: f64, t54512: f64, t8589: f64, t938: f64) -> f64 {
    let t55796 = 7.0_f64 / 1152.0_f64 * t54491;
    let t55807 = 7.0_f64 / 72.0_f64 * t4414 * t14954;
    let t55809 = 7.0_f64 / 72.0_f64 * t4414 * t14981;
    let t55831 = 7.0_f64 / 144.0_f64 * t840 * t15004;
    let t55833 = 7.0_f64 / 72.0_f64 * t54504;
    let t55836 = t55796 + t2408 * t2409 * t8589 * t14200 / 48.0_f64 + t3066 * t2409 * t3067 * t15081 * t938 / 24.0_f64 - t55807 - t55809 + t3066 * t2409 * t12213 * t14272 / 24.0_f64 + t2408 * t2409 * t2376 * t4110 * t2494 / 24.0_f64 + t3066 * t2409 * t3067 * t4110 * t3306 / 24.0_f64 + t2408 * t2409 * t2376 * t14240 * t1105 / 48.0_f64 - t54496 / 12.0_f64 + t55831 - t54502 / 384.0_f64 + t55833 + t54508 / 192.0_f64 + t54512 / 384.0_f64;
    t55836
}
