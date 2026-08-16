//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta851(t2349: f64, t656: f64, t10227: f64, t97: f64, t10241: f64, t105: f64, t4273: f64, t588: f64, t2289: f64, t4288: f64, t13455: f64, t625: f64, t14619: f64, t750: f64, t4398: f64, t9372: f64, t1469: f64, t2608: f64, t4401: f64, t606: f64, t14425: f64, t705: f64, t39454: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49774, t49777, t49787, t49804, t49817, t49819) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991(t2349, t656, t10227, t97, t10241, t105, t4273, t588, t2289, t4288, t13455, t625);
        let (t49864, t49866, t49876, t49880, t49887, t49897) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2992(t14619, t750, t4398, t9372, t1469, t2608, t4401, t606, t14425, t705, t39454, t9387);
    (t49774, t49777, t49787, t49804, t49817, t49819, t49864, t49866, t49876, t49880, t49887, t49897)
}
