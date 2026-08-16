//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2455;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2456;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta586(t6071: f64, t72: f64, t686: f64, t2465: f64, t213: f64, t6041: f64, t6048: f64, t10995: f64, t10987: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11030: f64, t15018: f64, t15047: f64, t15050: f64, t887: f64, t6072: f64, t779: f64, t689: f64, t1580: f64, t4321: f64, t6042: f64, t786: f64, t789: f64, t6049: f64, t14987: f64, t4481: f64, t11040: f64, t15011: f64, t15062: f64, t15063: f64, t2765: f64, t4474: f64, t4487: f64, t4534: f64, t18322: f64, t18791: f64, t10563: f64, t10566: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t18392: f64, t18535: f64, t18536: f64, t18537: f64, t18538: f64, t18541: f64, t18543: f64, t18546: f64, t18548: f64, t18549: f64, t18552: f64, t198: f64, t207: f64, t2403: f64, t4343: f64, t4546: f64, t765: f64, t892: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18796, t18797, t18800, t18804, t18805, t18810) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2455(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
        let (t18811, t18814, t18821, t18825, t18836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2456(t6072, t779, t689, t1580, t4321, t6042, t786, t789, t6049, t14987, t4481, t11040, t15011, t15062, t15063, t2765, t4474, t4487, t4534);
        let (t18838, t18848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2457(t18322, t18791, t18810, t18836, t10563, t10566, t14324, t14343, t14345, t14372, t18392, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t198, t207, t2403, t4343, t4546, t765, t892, t9394);
    (t18796, t18797, t18800, t18804, t18805, t18811, t18814, t18821, t18825, t18838, t18848)
}
