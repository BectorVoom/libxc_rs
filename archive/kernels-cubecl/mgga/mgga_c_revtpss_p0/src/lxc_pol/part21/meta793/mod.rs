//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2867;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2869;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta793<F: Float>(t52035: F, t52037: F, t2852: F, t373: F, t51957: F, t51959: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52090: F, t923: F, t41406: F, t52065: F, t52068: F, t2439: F, t4628: F, t1606: F, t9303: F, t916: F, t41441: F, t51889: F, t51919: F, t51949: F, t51975: F, t52009: F, t52043: F, t964: F, t973: F, t981: F, t11467: F, t1633: F, t41235: F, t41238: F, t11465: F, t3015: F, t4707: F, t11299: F, t15389: F, t2875: F, t11379: F, t1610: F, t2874: F, t11300: F, t15396: F, t41588: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52091, t52092, t52112) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2867::<F>(t52035, t52037, t2852, t373, t51957, t51959);
        let t52114 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868::<F>(t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52091, t52092, t52112);
        let (t52115, t52116, t52118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2869::<F>(t52090, t52114, t923, t41406, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068);
        let (t52126, t52128, t52130, t52134) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870::<F>(t2439, t4628, t1606, t9303, t52115, t916, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t41441, t52112);
        let (t52137, t52141, t52146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871::<F>(t51889, t51919, t51949, t51975, t52009, t52043, t52118, t52134, t964, t973, t981, t11467, t1633, t41235, t41238);
        let (t52150, t52153, t52156, t52159) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2872::<F>(t11465, t3015, t4707, t981, t11299, t15389, t2875, t11379, t1610, t2874, t11300, t15396, t41588);
    (t52112, t52116, t52126, t52128, t52130, t52137, t52141, t52146, t52150, t52153, t52156, t52159)
}
