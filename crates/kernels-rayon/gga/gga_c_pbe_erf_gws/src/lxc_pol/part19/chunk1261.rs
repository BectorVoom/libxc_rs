//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1261/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1261(t14928: f64, t840: f64, t53873: f64, t15018: f64, t53896: f64, t54014: f64, t54052: f64, t54072: f64, t54087: f64, t54102: f64, t54113: f64, t54117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55385 = 7.0_f64 / 144.0_f64 * t840 * t14928;
    let t55403 = 7.0_f64 / 576.0_f64 * t53873;
    let t55420 = 7.0_f64 / 144.0_f64 * t840 * t15018;
    let t55421 = 7.0_f64 / 36.0_f64 * t53896;
    let t55432 = 7.0_f64 / 288.0_f64 * t54014;
    let t55452 = 7.0_f64 / 96.0_f64 * t54052;
    let t55460 = 7.0_f64 / 72.0_f64 * t54072;
    let t55467 = 7.0_f64 / 72.0_f64 * t54087;
    let t55473 = 7.0_f64 / 36.0_f64 * t54102;
    let t55480 = 7.0_f64 / 144.0_f64 * t54113;
    let t55482 = 7.0_f64 / 144.0_f64 * t54117;
    (t55385, t55403, t55420, t55421, t55432, t55452, t55460, t55467, t55473, t55480, t55482)
}
