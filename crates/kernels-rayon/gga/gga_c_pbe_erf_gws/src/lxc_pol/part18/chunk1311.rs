//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1311/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1311(t14627: f64, t15139: f64, t2408: f64, t2409: f64, t26604: f64, t51572: f64, t53704: f64, t53726: f64, t53728: f64, t56740: f64, t56743: f64, t56745: f64, t56747: f64, t56753: f64, t56757: f64, t56761: f64, t56769: f64, t56773: f64, t8589: f64) -> f64 {
    let t56775 = -t56740 / 96.0_f64 - t56743 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t56745 - 7.0_f64 / 2304.0_f64 * t56747 - 35.0_f64 / 432.0_f64 * t51572 - t53704 + t56753 / 768.0_f64 + t56757 / 768.0_f64 - t56761 / 3072.0_f64 + t2408 * t2409 * t8589 * t14627 / 24.0_f64 + t26604 * t15139 / 96.0_f64 + 7.0_f64 / 36.0_f64 * t56769 + t56773 / 96.0_f64 - t53726 + t53728;
    t56775
}
