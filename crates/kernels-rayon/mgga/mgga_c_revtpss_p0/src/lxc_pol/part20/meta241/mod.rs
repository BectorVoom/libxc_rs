//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1045;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1046;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta241(t11028: f64, t780: f64, t2439: f64, t10910: f64, t225: f64, t2772: f64, t779: f64, t689: f64, t781: f64, t9292: f64, t861: f64, t867: f64, t786: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t11004: f64, t11010: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t11026: f64, t213: f64, t257: f64, t2765: f64, t2829: f64, t865: f64, t11002: f64, t2408: f64, t890: f64, t2410: f64, t261: f64, t2411: f64, t2832: f64, t892: f64, t10552: f64, t10554: f64, t10557: f64, t10560: f64, t10562: f64, t10564: f64, t10627: f64, t1940: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2404: f64, t2430: f64, t262: f64, t4541: f64, t775: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11029, t11030, t11032, t11036, t11037, t11040, t11043) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1045(t11028, t780, t2439, t10910, t225, t2772, t779, t689, t781, t9292, t861, t867);
        let (t11044, t11050, t11053) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1046(t11043, t786, t2467, t2828, t676, t123, t2465, t11004, t11010, t11013, t11017, t11019, t11022, t11026, t11030, t11032, t11037, t11040, t213, t257, t2765, t2772, t2829, t865);
        let (t11054, t11061, t11064, t11071, t11075, t11082) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1047(t11002, t11053, t2408, t890, t2410, t261, t2411, t2832, t892, t10552, t10554, t10557, t10560, t10562, t10564, t10627, t1940, t198, t207, t2394, t2403, t2404, t2430, t262, t4541, t775, t9394);
    (t11029, t11032, t11036, t11043, t11044, t11050, t11054, t11061, t11064, t11071, t11075, t11082)
}
