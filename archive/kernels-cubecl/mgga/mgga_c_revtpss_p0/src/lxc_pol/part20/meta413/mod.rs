//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1525;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1526;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta413<F: Float>(t11267: F, t3123: F, t3147: F, t3229: F, t3141: F, t3144: F, t1036: F, t11922: F, t12016: F, t3115: F, t11638: F, t3127: F, t3172: F, t11683: F, t11710: F, t3091: F, t11671: F, t3278: F, t12020: F, t3168: F, t11245: F, t42668: F, t11628: F, t42860: F, t42866: F, t11631: F, t42871: F, t3154: F, t2434: F, t246: F, t1041: F, t1046: F, t10326: F, t1042: F, t1047: F, t11252: F, t12021: F, t3097: F, t3136: F, t3150: F, t3155: F, t42386: F, t42870: F, t4872: F, t999: F, t11256: F, t11258: F, t11727: F, t3188: F, t12004: F, t3111: F, t1011: F, t11165: F, t15987: F, t11156: F, t15993: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42934, t42939, t42943, t42947, t42962) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1525::<F>(t11267, t3123, t3147, t3229, t3141, t3144, t1036, t11922, t12016, t3115, t11638, t3127, t3172);
        let (t42965, t42967, t42970, t42973, t42977) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1526::<F>(t11683, t11710, t3091, t11671, t3278, t12020, t3168, t11245, t42668, t11628, t42860, t42866);
        let (t42994, t42998) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527::<F>(t11631, t42871, t3144, t42860, t42866, t3154, t2434, t246, t1041, t1046, t10326, t1042, t1047, t11252, t12021, t3097, t3127, t3136, t3150, t3155, t42386, t42870, t42962, t42965, t42967, t42970, t42973, t42977, t4872, t999);
        let (t43003, t43017, t43019, t43029, t43032) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1528::<F>(t11256, t11258, t3172, t11727, t3188, t12004, t3111, t1011, t11165, t15987, t11156, t15993);
    (t42934, t42939, t42943, t42947, t42994, t42998, t43003, t43017, t43019, t43029, t43032)
}
