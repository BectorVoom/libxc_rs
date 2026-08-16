//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1525;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1526;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta413(t11267: f64, t3123: f64, t3147: f64, t3229: f64, t3141: f64, t3144: f64, t1036: f64, t11922: f64, t12016: f64, t3115: f64, t11638: f64, t3127: f64, t3172: f64, t11683: f64, t11710: f64, t3091: f64, t11671: f64, t3278: f64, t12020: f64, t3168: f64, t11245: f64, t42668: f64, t11628: f64, t42860: f64, t42866: f64, t11631: f64, t42871: f64, t3154: f64, t2434: f64, t246: f64, t1041: f64, t1046: f64, t10326: f64, t1042: f64, t1047: f64, t11252: f64, t12021: f64, t3097: f64, t3136: f64, t3150: f64, t3155: f64, t42386: f64, t42870: f64, t4872: f64, t999: f64, t11256: f64, t11258: f64, t11727: f64, t3188: f64, t12004: f64, t3111: f64, t1011: f64, t11165: f64, t15987: f64, t11156: f64, t15993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42934, t42939, t42943, t42947, t42962) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1525(t11267, t3123, t3147, t3229, t3141, t3144, t1036, t11922, t12016, t3115, t11638, t3127, t3172);
        let (t42965, t42967, t42970, t42973, t42977) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1526(t11683, t11710, t3091, t11671, t3278, t12020, t3168, t11245, t42668, t11628, t42860, t42866);
        let (t42994, t42998) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527(t11631, t42871, t3144, t42860, t42866, t3154, t2434, t246, t1041, t1046, t10326, t1042, t1047, t11252, t12021, t3097, t3127, t3136, t3150, t3155, t42386, t42870, t42962, t42965, t42967, t42970, t42973, t42977, t4872, t999);
        let (t43003, t43017, t43019, t43029, t43032) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1528(t11256, t11258, t3172, t11727, t3188, t12004, t3111, t1011, t11165, t15987, t11156, t15993);
    (t42934, t42939, t42943, t42947, t42994, t42998, t43003, t43017, t43019, t43029, t43032)
}
