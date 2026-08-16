//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1930;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta478(t20090: f64, t3117: f64, t1651: f64, t2857: f64, t4181: f64, t3092: f64, t2852: f64, t11703: f64, t19611: f64, t4910: f64, t11859: f64, t15850: f64, t16095: f64, t16165: f64, t16218: f64, t16220: f64, t1675: f64, t20075: f64, t20079: f64, t20083: f64, t3091: f64, t3115: f64, t4837: f64, t11264: f64, t11675: f64, t11818: f64, t11875: f64, t11927: f64, t15583: f64, t15618: f64, t15662: f64, t15707: f64, t15862: f64, t15865: f64, t15892: f64, t15926: f64, t15942: f64, t19622: f64, t19626: f64, t19636: f64, t19641: f64, t19645: f64, t19685: f64, t19729: f64, t19763: f64, t19797: f64, t19813: f64, t19841: f64, t19885: f64, t19895: f64, t19901: f64, t19923: f64, t19950: f64, t19989: f64, t20012: f64, t20036: f64, t20073: f64, t3127: f64, t3241: f64, t4783: f64, t4825: f64, t4899: f64, t4907: f64, t6268: f64, t6285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20091, t20095, t20096, t20100, t20101, t20104, t20105, t20108) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1930(t20090, t3117, t1651, t2857, t4181, t3092, t2852, t11703, t19611, t4910, t11859, t15850, t16095, t16165, t16218, t16220, t1675, t20075, t20079, t20083, t3091, t3115, t4837);
        let t20112 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1931(t11264, t11675, t11818, t11859, t11875, t11927, t15583, t15618, t15662, t15707, t15862, t15865, t15892, t15926, t15942, t19622, t19626, t19636, t19641, t19645, t19685, t19729, t19763, t19797, t19813, t19841, t19885, t19895, t19901, t19923, t19950, t19989, t20012, t20036, t20073, t20108, t3091, t3127, t3241, t4783, t4825, t4899, t4907, t6268, t6285);
    (t20091, t20095, t20096, t20100, t20101, t20104, t20105, t20112)
}
