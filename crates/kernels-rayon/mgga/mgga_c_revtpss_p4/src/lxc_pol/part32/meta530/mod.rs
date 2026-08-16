//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta530(t25410: f64, t93320: f64, t7063: f64, t860: f64, t25374: f64, t11007: f64, t1955: f64, t7056: f64, t93189: f64, t93169: f64, t1113: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1835(t25410, t93320, t7063, t860, t25374, t11007, t1955, t7056, t93189, t93169, t1113, t2411);
    (t93321, t93342, t93349, t93364, t93371, t93374, t93377, t94245)
}
