//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2080;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta602(t26093: f64, t575: f64, t116: f64, t25832: f64, t26133: f64, t571: f64, t2327: f64, t7724: f64, t27833: f64, t7316: f64, t13426: f64, t7003: f64, t18227: f64, t25861: f64, t4248: f64, t3813: f64, t651: f64, t7741: f64, t28159: f64, t18153: f64, t1936: f64, t670: f64, t6982: f64, t13429: f64, t13521: f64, t13532: f64, t13540: f64, t1519: f64, t2007: f64, t2320: f64, t2328: f64, t2331: f64, t25805: f64, t27830: f64, t28030: f64, t4297: f64, t508: f64, t649: f64, t671: f64, t6985: f64, t7883: f64, t92737: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95127, t95137, t95180, t97593, t97604, t97606) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2079(t26093, t575, t116, t25832, t26133, t571, t2327, t7724, t27833, t7316, t13426, t7003);
        let (t97608, t97610, t97617, t97622, t97629, t97632) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2080(t18227, t7003, t25861, t4248, t3813, t651, t7741, t116, t28159, t18153, t1936, t670, t6982);
        let t97635 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081(t13429, t13521, t13532, t13540, t1519, t2007, t2320, t2328, t2331, t25805, t27830, t28030, t4297, t508, t649, t671, t6985, t7883, t92737, t97593, t97604, t97606, t97608, t97610, t97617, t97622, t97629, t97632);
    (t95127, t95137, t95180, t97593, t97622, t97632, t97635)
}
