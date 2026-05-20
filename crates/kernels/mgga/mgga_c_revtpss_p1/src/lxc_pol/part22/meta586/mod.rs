//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2455;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2456;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta586<F: Float>(t6071: F, t72: F, t686: F, t2465: F, t213: F, t6041: F, t6048: F, t10995: F, t10987: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11030: F, t15018: F, t15047: F, t15050: F, t887: F, t6072: F, t779: F, t689: F, t1580: F, t4321: F, t6042: F, t786: F, t789: F, t6049: F, t14987: F, t4481: F, t11040: F, t15011: F, t15062: F, t15063: F, t2765: F, t4474: F, t4487: F, t4534: F, t18322: F, t18791: F, t10563: F, t10566: F, t14324: F, t14343: F, t14345: F, t14372: F, t18392: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t18548: F, t18549: F, t18552: F, t198: F, t207: F, t2403: F, t4343: F, t4546: F, t765: F, t892: F, t9394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18796, t18797, t18800, t18804, t18805, t18810) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2455::<F>(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let (t18811, t18814, t18821, t18825, t18836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2456::<F>(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let (t18838, t18848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2457::<F>(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
    (t18796, t18797, t18800, t18804, t18805, t18811, t18814, t18821, t18825, t18838, t18848)
}
