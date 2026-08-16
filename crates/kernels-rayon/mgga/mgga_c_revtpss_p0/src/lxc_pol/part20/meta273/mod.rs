//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1124;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1125;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1126;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta273(t11772: f64, t3088: f64, t3114: f64, t3128: f64, t372: f64, t3096: f64, t1024: f64, t3230: f64, t11213: f64, t225: f64, t366: f64, t11223: f64, t1053: f64, t3223: f64, t3215: f64, t3224: f64, t1011: f64, t1028: f64, t11753: f64, t11756: f64, t11759: f64, t11763: f64, t11767: f64, t3208: f64, t3211: f64, t3220: f64, t3238: f64, t3241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11773, t11774) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1124(t11772, t3088, t3114);
        let (t11775, t11776, t11779, t11782) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1125(t3128, t372, t3096, t1024, t3230, t11213, t225);
        let (t11783, t11788) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1126(t11782, t366, t11223, t225);
        let (t11789, t11792, t11799) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1127(t11788, t366, t1053, t3223, t3215, t3224, t1011, t1028, t11753, t11756, t11759, t11763, t11767, t11774, t11776, t11779, t11783, t3208, t3211, t3220, t3238, t3241);
    (t11773, t11774, t11775, t11776, t11779, t11782, t11783, t11788, t11789, t11792, t11799)
}
