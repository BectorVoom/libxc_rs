//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1651;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1652;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta454<F: Float>(t12712: F, t471: F, t6688: F, t3720: F, t1774: F, t3367: F, t4181: F, t3626: F, t6622: F, t73: F, t5352: F, t20956: F, t5333: F, t17934: F, t5330: F, t5327: F, t5362: F, t12809: F, t12853: F, t17290: F, t17386: F, t17417: F, t17425: F, t17605: F, t17729: F, t17753: F, t1791: F, t3718: F, t5343: F, t5402: F, t1803: F, t5326: F, t12297: F, t12610: F, t16706: F, t16708: F, t16711: F, t16713: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F, F, F, F, F) {
        let (t21030, t21037, t21040, t21042, t21045) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1651::<F>(t12712, t471, t6688, t3720, t1774, t3367, t4181, t3626, t6622, t73, t5352, t20956, t5333);
        let (t21046, t21057) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1652::<F>(t21045, t3720, t17934, t5330, t5327, t5362, t12809, t12853, t17290, t17386, t17417, t17425, t17605, t17729, t17753, t1791, t21030, t21037, t21042, t3718, t5343, t5402);
        let (t21063, t21082) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1653::<F>(t1803, t5326, t12297, t12610, t16706, t16708, t16711, t16713, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
    (t21030, t21037, t21040, t21042, t21046, t21057, t21063, t21082)
}
