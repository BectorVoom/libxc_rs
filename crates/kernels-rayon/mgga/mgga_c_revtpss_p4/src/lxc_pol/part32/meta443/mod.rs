//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1610;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1611;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1612;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta443(t22046: f64, t3936: f64, t3938: f64, t5659: f64, t5673: f64, t5674: f64, t1399: f64, t125: f64, t6836: f64, t9955: f64, t1413: f64, t6816: f64, t547: f64, t807: f64, t4011: f64, t1353: f64, t6883: f64, t800: f64, t13832: f64, t13851: f64, t13858: f64, t3934: f64, t3944: f64, t9739: f64, t9742: f64, t9766: f64, t13784: f64, t13790: f64, t13789: f64, t13880: f64, t13943: f64, t13949: f64, t13954: f64, t13956: f64, t5671: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64, t6871: f64, t9962: f64, t22016: f64, t5675: f64, t6849: f64, t1872: f64, t5591: f64, t13804: f64, t13959: f64, t13987: f64, t13988: f64, t14001: f64, t14007: f64, t9748: f64, t9804: f64, t9847: f64, t9910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1610(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let (t22129, t22140) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1611(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let (t22146, t22153) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1612(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
        let (t22159, t22163, t22176) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1613(t6871, t9962, t22016, t22046, t5673, t5675, t1353, t6849, t800, t1872, t5591, t13804, t13959, t13987, t13988, t14001, t14007, t3944, t5671, t9748, t9804, t9847, t9910);
    (t22107, t22111, t22115, t22120, t22125, t22129, t22140, t22146, t22153, t22159, t22163, t22176)
}
