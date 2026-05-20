//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta934 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta934<F: Float>(t17769: F, t3647: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F, t1789: F, t2434: F, t1261: F, t16746: F, t247: F, t3634: F, t1012: F, t44958: F, t13026: F, t140: F, t1222: F, t16715: F, t1224: F, t5052: F, t697: F, t12915: F, t17344: F, t17345: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57451, t57463, t57465, t57471, t57478) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165::<F>(t17769, t3647, t1235, t371, t5318, t676, t225, t56331, t1789, t2434, t1261, t16746, t247, t3634);
        let (t57480, t57484, t57486, t57490, t57508) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166::<F>(t1012, t44958, t13026, t140, t1222, t16715, t1224, t5052, t697, t12915, t17344, t17345, t247);
    (t57451, t57463, t57465, t57471, t57478, t57480, t57484, t57486, t57490, t57508)
}
