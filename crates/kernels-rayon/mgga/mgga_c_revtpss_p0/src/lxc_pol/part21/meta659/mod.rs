//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2451;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta659(t11880: f64, t3241: f64, t1011: f64, t1016: f64, t2438: f64, t3237: f64, t697: f64, t1010: f64, t10345: f64, t11883: f64, t3244: f64, t11766: f64, t140: f64, t1014: f64, t11150: f64, t1003: f64, t11735: f64, t221: f64, t345: f64, t346: f64, t624: f64, t1007: f64, t11738: f64, t3080: f64, t3083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42712, t42716, t42719, t42721, t42724, t42727) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2451(t11880, t3241, t1011, t1016, t2438, t3237, t697, t1010, t10345, t11883, t3244, t11766, t140);
        let (t42731, t42740, t42745, t42754, t42756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2452(t1014, t11150, t1003, t11735, t221, t345, t346, t624, t1007, t11738, t3080, t3083);
    (t42712, t42716, t42719, t42721, t42724, t42727, t42731, t42740, t42745, t42754, t42756)
}
