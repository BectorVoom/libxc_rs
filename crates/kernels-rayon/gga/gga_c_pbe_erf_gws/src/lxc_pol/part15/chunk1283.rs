//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1283/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1283(t14404: f64, t19704: f64, t51756: f64, t51758: f64, t51769: f64, t51771: f64, t51781: f64, t51788: f64, t53843: f64, t53846: f64, t53848: f64, t53852: f64, t53856: f64, t53862: f64, t53867: f64, t53870: f64) -> f64 {
    let t53872 = 7.0_f64 / 144.0_f64 * t51756 - 7.0_f64 / 72.0_f64 * t51758 + 7.0_f64 / 48.0_f64 * t51769 - t53843 / 8.0_f64 - 7.0_f64 / 2304.0_f64 * t51771 + t53846 / 24.0_f64 + t53848 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t51781 + 7.0_f64 / 288.0_f64 * t51788 - 35.0_f64 / 432.0_f64 * t53852 + t53856 / 384.0_f64 + t19704 * t14404 / 48.0_f64 + t53862 / 192.0_f64 + 5.0_f64 / 192.0_f64 * t53867 - t53870 / 1536.0_f64;
    t53872
}
