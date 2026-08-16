//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1770;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta495(t2055: f64, t5517: f64, t72: f64, t8094: f64, t686: f64, t25878: f64, t25895: f64, t1882: f64, t543: f64, t7506: f64, t7301: f64, t27884: f64, t7515: f64, t25921: f64, t26232: f64, t26235: f64, t26238: f64, t26251: f64, t26253: f64, t26263: f64, t26266: f64, t26268: f64, t26272: f64, t7295: f64, t8100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28760, t28779, t28780) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1770(t2055, t5517, t72, t8094, t686);
        let (t28781, t28783, t28791, t28792, t28796, t28799) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1771(t25878, t28780, t25895, t1882, t543, t7506, t7301, t27884, t7515, t25921, t26232, t26235, t26238, t26251, t26253, t26263, t26266, t26268, t26272, t7295, t8100);
    (t28760, t28779, t28780, t28781, t28783, t28791, t28792, t28796, t28799)
}
