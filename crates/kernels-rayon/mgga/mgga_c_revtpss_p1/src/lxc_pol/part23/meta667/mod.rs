//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta667(t11240: f64, t3144: f64, t42646: f64, t11239: f64, t989: f64, t11629: f64, t11874: f64, t16048: f64, t12046: f64, t15905: f64, t994: f64, t1011: f64, t1016: f64, t2438: f64, t1014: f64, t11150: f64, t1003: f64, t11735: f64, t221: f64, t345: f64, t346: f64, t624: f64, t11858: f64, t1065: f64, t215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42648, t42668, t42669, t42675, t42690, t42716) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399(t11240, t3144, t42646, t11239, t989, t11629, t11874, t16048, t12046, t15905, t994, t1011, t1016, t2438);
        let (t42731, t42740, t42745, t42765, t42778) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2400(t1014, t11150, t1003, t11735, t221, t345, t346, t624, t11858, t16048, t1065, t215);
    (t42648, t42668, t42669, t42675, t42690, t42716, t42731, t42740, t42745, t42765, t42778)
}
