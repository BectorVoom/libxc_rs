//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 874/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk874(t1017: f64, t5219: f64, t1823: f64, t5218: f64, t1802: f64, t589: f64, t1828: f64, t7062: f64, t5349: f64, t7461: f64, t7466: f64, t7472: f64, t7474: f64, t7476: f64, t7478: f64, t7479: f64, t7480: f64, t7482: f64, t7489: f64, t7494: f64, t7498: f64, t7504: f64, t7509: f64) -> (f64, f64, f64, f64) {
    let t7510 = t5219 * t1017;
    let t7511 = t7510 * t1823;
    let t7513 = 16.0_f64 / 45.0_f64 * t5218 * t7511;
    let t7514 = t589 * t1802;
    let t7515 = t7514 * t1017;
    let t7516 = t7515 * t1828;
    let t7518 = 16.0_f64 / 45.0_f64 * t7062 * t7516;
    let t7519 = 8.0_f64 / 45.0_f64 * t5349;
    let t7520 = -t7461 + t7466 - t7472 - t7474 - t7476 - t7478 + t7479 + t7480 + t7482 - t7489 + t7494 - t7498 - t7504 + t7509 - t7513 + t7518 + t7519;
    (t7513, t7518, t7519, t7520)
}
