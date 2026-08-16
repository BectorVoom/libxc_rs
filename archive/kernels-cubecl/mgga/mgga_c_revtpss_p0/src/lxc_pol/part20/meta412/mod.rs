//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1522;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta412<F: Float>(t3075: F, t3151: F, t11875: F, t11876: F, t11922: F, t11991: F, t3111: F, t1062: F, t11903: F, t11988: F, t3188: F, t11173: F, t999: F, t11255: F, t42668: F, t1068: F, t11259: F, t247: F, t3116: F, t3117: F, t3162: F, t42883: F, t42886: F, t42889: F, t42892: F, t4837: F, t1036: F, t42860: F, t42866: F, t357: F, t42871: F, t11263: F, t3124: F, t11262: F, t3150: F, t3156: F, t3161: F, t3163: F) -> (F, F, F, F, F, F, F, F) {
        let (t42894, t42900, t42902, t42904, t42907, t42909) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1522::<F>(t3075, t3151, t11875, t11876, t11922, t11991, t3111, t1062, t11903, t11988, t3188, t11173, t999);
        let t42917 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523::<F>(t11255, t42668, t1068, t11259, t11875, t247, t3116, t3117, t3162, t42883, t42886, t42889, t42892, t42894, t42900, t42902, t42904, t42907, t42909, t4837);
        let (t42920, t42921, t42926, t42929, t42932) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1524::<F>(t1036, t42860, t42866, t357, t42871, t11263, t3124, t11262, t3150, t3156, t3161, t3163);
    (t42894, t42909, t42917, t42920, t42921, t42926, t42929, t42932)
}
