//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1124;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1125;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1126;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta273<F: Float>(t11772: F, t3088: F, t3114: F, t3128: F, t372: F, t3096: F, t1024: F, t3230: F, t11213: F, t225: F, t366: F, t11223: F, t1053: F, t3223: F, t3215: F, t3224: F, t1011: F, t1028: F, t11753: F, t11756: F, t11759: F, t11763: F, t11767: F, t3208: F, t3211: F, t3220: F, t3238: F, t3241: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11773, t11774) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1124::<F>(t11772, t3088, t3114);
        let (t11775, t11776, t11779, t11782) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1125::<F>(t3128, t372, t3096, t1024, t3230, t11213, t225);
        let (t11783, t11788) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1126::<F>(t11782, t366, t11223, t225);
        let (t11789, t11792, t11799) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127::<F>(t11788, t366, t1053, t3223, t3215, t3224, t1011, t1028, t11753, t11756, t11759, t11763, t11767, t11774, t11776, t11779, t11783, t3208, t3211, t3220, t3238, t3241);
    (t11773, t11774, t11775, t11776, t11779, t11782, t11783, t11788, t11789, t11792, t11799)
}
