//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1039 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3630;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1039(t68707: f64, t68709: f64, t68711: f64, t68714: f64, t68716: f64, t68718: f64, t68723: f64, t68725: f64, t68727: f64, t68730: f64, t68733: f64, t68735: f64, t16677: f64, t5192: f64, t1196: f64, t12485: f64, t3524: f64, t6534: f64, t20400: f64, t3535: f64, t17164: f64, t20391: f64, t3531: f64, t3427: f64, t3433: f64, t6439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t68736 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3630(t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733, t68735);
        let (t68738, t68742, t68744, t68746, t68748, t68751) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631(t16677, t5192, t1196, t12485, t3524, t6534, t20400, t3535, t17164, t20391, t3531, t3427, t3433, t6439);
    (t68736, t68738, t68742, t68744, t68746, t68748, t68751)
}
