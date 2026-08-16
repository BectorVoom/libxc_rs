//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2091;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta542(t22125: f64, t547: f64, t807: f64, t4011: f64, t6836: f64, t1353: f64, t6883: f64, t800: f64, t13832: f64, t13851: f64, t13858: f64, t22107: f64, t22111: f64, t22115: f64, t22120: f64, t3934: f64, t3944: f64, t9739: f64, t9742: f64, t9766: f64, t13784: f64, t13790: f64, t13789: f64, t13880: f64, t13943: f64, t13949: f64, t13954: f64, t13956: f64, t5671: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22126, t22127, t22129, t22130, t22131, t22135, t22140) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2091(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let (t22145, t22146, t22153) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2092(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
    (t22126, t22127, t22129, t22130, t22131, t22135, t22140, t22145, t22146, t22153)
}
