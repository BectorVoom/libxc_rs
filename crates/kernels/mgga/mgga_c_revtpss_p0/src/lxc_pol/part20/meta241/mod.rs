//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1045;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1046;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta241<F: Float>(t11028: F, t780: F, t2439: F, t10910: F, t225: F, t2772: F, t779: F, t689: F, t781: F, t9292: F, t861: F, t867: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t11004: F, t11010: F, t11013: F, t11017: F, t11019: F, t11022: F, t11026: F, t213: F, t257: F, t2765: F, t2829: F, t865: F, t11002: F, t2408: F, t890: F, t2410: F, t261: F, t2411: F, t2832: F, t892: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t10564: F, t10627: F, t1940: F, t198: F, t207: F, t2394: F, t2403: F, t2404: F, t2430: F, t262: F, t4541: F, t775: F, t9394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11029, t11030, t11032, t11036, t11037, t11040, t11043) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1045::<F>(t11028, t780, t2439, t10910, t225, t2772, t779, t689, t781, t9292, t861, t867);
        let (t11044, t11050, t11053) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1046::<F>(t11043, t786, t2467, t2828, t676, t123, t2465, t11004, t11010, t11013, t11017, t11019, t11022, t11026, t11030, t11032, t11037, t11040, t213, t257, t2765, t2772, t2829, t865);
        let (t11054, t11061, t11064, t11071, t11075, t11082) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1047::<F>(t11002, t11053, t2408, t890, t2410, t261, t2411, t2832, t892, t10552, t10554, t10557, t10560, t10562, t10564, t10627, t1940, t198, t207, t2394, t2403, t2404, t2430, t262, t4541, t775, t9394);
    (t11029, t11032, t11036, t11043, t11044, t11050, t11054, t11061, t11064, t11071, t11075, t11082)
}
