//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1369/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1369(t1185: f64, t14419: f64, t15272: f64, t15325: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t3068: f64, t4155: f64, t54480: f64, t54482: f64, t54598: f64, t54599: f64, t57495: f64, t57497: f64, t57500: f64, t57506: f64, t57509: f64, t57514: f64, t57516: f64, t57518: f64, t57534: f64, t6781: f64, t810: f64, t8654: f64, t938: f64) -> f64 {
    let t57536 = t8654 * t1185 * t14419 / 24.0_f64 + t57495 / 768.0_f64 - t57497 / 96.0_f64 - t57500 / 192.0_f64 + t54598 * t54599 * t4155 * t3068 / 4.0_f64 - t57506 / 48.0_f64 - t57509 / 96.0_f64 + t57514 / 96.0_f64 + 7.0_f64 / 4608.0_f64 * t57516 + t54480 + t54482 + 7.0_f64 / 288.0_f64 * t57518 + t3066 * t2409 * t3067 * t15272 * t938 / 48.0_f64 + t2408 * t2409 * t2376 * t15272 * t810 / 48.0_f64 + t2408 * t2409 * t6781 * t15325 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t57534;
    t57536
}
