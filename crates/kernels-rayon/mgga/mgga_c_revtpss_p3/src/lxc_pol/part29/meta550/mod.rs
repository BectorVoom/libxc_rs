//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta550(t25904: f64, t96245: f64, t94471: f64, t94473: f64, t94476: f64, t94483: f64, t94522: f64, t94525: f64, t94568: f64, t94570: f64, t26334: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96298, t96321, t96322, t96323, t96326, t96341, t96342, t96358, t96359, t96370) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1887(t25904, t96245, t94471, t94473, t94476, t94483, t94522, t94525, t94568, t94570, t26334, t686, t72);
    (t96298, t96321, t96322, t96323, t96326, t96341, t96342, t96358, t96359, t96370)
}
