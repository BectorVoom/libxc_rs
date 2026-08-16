//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1847;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta537(t45963: f64, t7342: f64, t10301: f64, t26178: f64, t2247: f64, t239: f64, t38: f64, t6960: f64, t25163: f64, t7348: f64, t26205: f64, t6963: f64, t45972: f64, t10309: f64, t94973: f64, t530: f64, t7535: f64, t198: f64, t206: f64, t7427: f64, t25373: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95276, t95283, t95293, t95294, t95296, t95314) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1847(t45963, t7342, t10301, t26178, t2247, t239, t38, t6960, t25163, t7348, t26205, t6963);
        let (t95316, t95319, t95397, t95472, t95511, t95536) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1848(t45972, t7342, t10309, t26178, t94973, t530, t7535, t198, t206, t7427, t25373, t26550);
    (t95276, t95283, t95293, t95294, t95296, t95314, t95316, t95319, t95397, t95472, t95511, t95536)
}
