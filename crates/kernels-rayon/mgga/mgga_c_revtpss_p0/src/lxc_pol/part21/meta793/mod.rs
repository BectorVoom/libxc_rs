//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2867;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2869;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta793(t52035: f64, t52037: f64, t2852: f64, t373: f64, t51957: f64, t51959: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52090: f64, t923: f64, t41406: f64, t52065: f64, t52068: f64, t2439: f64, t4628: f64, t1606: f64, t9303: f64, t916: f64, t41441: f64, t51889: f64, t51919: f64, t51949: f64, t51975: f64, t52009: f64, t52043: f64, t964: f64, t973: f64, t981: f64, t11467: f64, t1633: f64, t41235: f64, t41238: f64, t11465: f64, t3015: f64, t4707: f64, t11299: f64, t15389: f64, t2875: f64, t11379: f64, t1610: f64, t2874: f64, t11300: f64, t15396: f64, t41588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52091, t52092, t52112) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2867(t52035, t52037, t2852, t373, t51957, t51959);
        let t52114 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868(t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52091, t52092, t52112);
        let (t52115, t52116, t52118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2869(t52090, t52114, t923, t41406, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068);
        let (t52126, t52128, t52130, t52134) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870(t2439, t4628, t1606, t9303, t52115, t916, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t41441, t52112);
        let (t52137, t52141, t52146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871(t51889, t51919, t51949, t51975, t52009, t52043, t52118, t52134, t964, t973, t981, t11467, t1633, t41235, t41238);
        let (t52150, t52153, t52156, t52159) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2872(t11465, t3015, t4707, t981, t11299, t15389, t2875, t11379, t1610, t2874, t11300, t15396, t41588);
    (t52112, t52116, t52126, t52128, t52130, t52137, t52141, t52146, t52150, t52153, t52156, t52159)
}
