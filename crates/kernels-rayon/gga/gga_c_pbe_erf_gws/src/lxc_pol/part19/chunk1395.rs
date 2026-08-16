//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1395/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1395(t4083: f64, t9955: f64, t12198: f64, t1105: f64, t353: f64, t4228: f64, t4386: f64, t54952: f64, t55796: f64, t55807: f64, t55809: f64, t57488: f64, t57495: f64, t57497: f64, t57500: f64, t57506: f64, t57509: f64, t57514: f64, t57516: f64, t57542: f64, t6793: f64, t8793: f64) -> f64 {
    let t58821 = t9955 * t4083;
    let t58823 = t12198 * t4083;
    let t58835 = t4386 * t353 * t4228 * t1105;
    let t58839 = 7.0_f64 / 36.0_f64 * t57488 + t57495 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t58821 + 7.0_f64 / 288.0_f64 * t58823 - t57497 / 48.0_f64 - t57500 / 96.0_f64 - t57506 / 24.0_f64 - t57509 / 48.0_f64 + t57514 / 48.0_f64 + 7.0_f64 / 2304.0_f64 * t57516 + t8793 * t54952 / 24.0_f64 + t6793 * t58835 / 24.0_f64 + t55796 - t55807 - t55809 + 7.0_f64 / 72.0_f64 * t57542;
    t58839
}
