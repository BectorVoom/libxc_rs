//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta572(t2028: f64, t28911: f64, t25894: f64, t97680: f64, t25875: f64, t96236: f64, t97688: f64, t26304: f64, t97705: f64, t96187: f64, t97685: f64, t136: f64, t2457: f64, t8103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102081, t102084, t102086, t102090, t102093, t102096, t102098, t102100) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1896(t2028, t28911, t25894, t97680, t25875, t96236, t97688, t26304, t97705, t96187, t97685, t136, t2457, t8103);
    (t102081, t102084, t102086, t102090, t102093, t102096, t102098, t102100)
}
