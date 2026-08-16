//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1651;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1652;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta454(t12712: f64, t471: f64, t6688: f64, t3720: f64, t1774: f64, t3367: f64, t4181: f64, t3626: f64, t6622: f64, t73: f64, t5352: f64, t20956: f64, t5333: f64, t17934: f64, t5330: f64, t5327: f64, t5362: f64, t12809: f64, t12853: f64, t17290: f64, t17386: f64, t17417: f64, t17425: f64, t17605: f64, t17729: f64, t17753: f64, t1791: f64, t3718: f64, t5343: f64, t5402: f64, t1803: f64, t5326: f64, t12297: f64, t12610: f64, t16706: f64, t16708: f64, t16711: f64, t16713: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21030, t21037, t21040, t21042, t21045) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1651(t12712, t471, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73, t5352, t20956, t5333);
        let (t21046, t21057) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1652(t21045, t3720, t17934, t5330, t5327, t5362, t12809, t12853, t17290, t17386, t17417, t17425, t17605, t17729, t17753, t1791, t21030, t21037, t21042, t3718, t5343, t5402);
        let (t21063, t21082) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1653(t1803, t5326, t12297, t12610, t16706, t16708, t16711, t16713, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
    (t21030, t21037, t21040, t21042, t21046, t21057, t21063, t21082)
}
