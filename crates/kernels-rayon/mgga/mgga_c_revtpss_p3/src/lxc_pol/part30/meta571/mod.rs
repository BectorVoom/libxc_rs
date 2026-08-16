//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta571(t2466: f64, t93329: f64, t25375: f64, t7015: f64, t9292: f64, t25411: f64, t93183: f64, t25431: f64, t93123: f64, t25387: f64, t93285: f64, t7063: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93330, t93331, t93334, t93335, t93337, t93339, t93341) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2020(t2466, t93329, t25375, t7015, t9292, t25411, t93183, t25431, t93123, t25387, t93285, t7063, t860);
    (t93330, t93331, t93334, t93335, t93337, t93339, t93341)
}
