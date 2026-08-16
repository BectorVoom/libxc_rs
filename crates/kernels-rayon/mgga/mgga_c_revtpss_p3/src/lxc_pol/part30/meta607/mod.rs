//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2070;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta607(t3813: f64, t651: f64, t7741: f64, t18153: f64, t1936: f64, t18163: f64, t7742: f64, t28063: f64, t4254: f64, t1937: f64, t75485: f64, t18227: f64, t6993: f64, t27126: f64, t7003: f64, t25856: f64, t7732: f64, t26090: f64, t7898: f64, t1353: f64, t28198: f64, t25082: f64, t28197: f64, t27833: f64, t7239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97617, t97629, t97639, t97641, t97643, t97645) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2070(t3813, t651, t7741, t18153, t1936, t18163, t7742, t28063, t4254, t1937, t75485, t18227, t6993);
        let (t97647, t97649, t97653, t97657, t97659) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2071(t27126, t7003, t25856, t7732, t26090, t7898, t1353, t28198, t25082, t28197, t27833, t7239);
    (t97617, t97629, t97639, t97641, t97643, t97645, t97647, t97649, t97653, t97657, t97659)
}
