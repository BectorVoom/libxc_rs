//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta575(t102218: f64, t25895: f64, t102204: f64, t94771: f64, t122: f64, t72: f64, t8085: f64, t25900: f64, t25899: f64, t28894: f64, t94921: f64, t94802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t102219, t102225, t102234, t102235, t102237, t102239, t102241) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1901(t102218, t25895, t102204, t94771, t122, t72, t8085, t25900, t25899, t28894, t94921, t94802);
    (t102219, t102225, t102234, t102235, t102237, t102239, t102241)
}
