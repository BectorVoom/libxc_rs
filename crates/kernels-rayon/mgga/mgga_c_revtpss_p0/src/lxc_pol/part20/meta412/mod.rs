//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1522;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta412(t3075: f64, t3151: f64, t11875: f64, t11876: f64, t11922: f64, t11991: f64, t3111: f64, t1062: f64, t11903: f64, t11988: f64, t3188: f64, t11173: f64, t999: f64, t11255: f64, t42668: f64, t1068: f64, t11259: f64, t247: f64, t3116: f64, t3117: f64, t3162: f64, t42883: f64, t42886: f64, t42889: f64, t42892: f64, t4837: f64, t1036: f64, t42860: f64, t42866: f64, t357: f64, t42871: f64, t11263: f64, t3124: f64, t11262: f64, t3150: f64, t3156: f64, t3161: f64, t3163: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42894, t42900, t42902, t42904, t42907, t42909) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1522(t3075, t3151, t11875, t11876, t11922, t11991, t3111, t1062, t11903, t11988, t3188, t11173, t999);
        let t42917 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523(t11255, t42668, t1068, t11259, t11875, t247, t3116, t3117, t3162, t42883, t42886, t42889, t42892, t42894, t42900, t42902, t42904, t42907, t42909, t4837);
        let (t42920, t42921, t42926, t42929, t42932) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1524(t1036, t42860, t42866, t357, t42871, t11263, t3124, t11262, t3150, t3156, t3161, t3163);
    (t42894, t42909, t42917, t42920, t42921, t42926, t42929, t42932)
}
