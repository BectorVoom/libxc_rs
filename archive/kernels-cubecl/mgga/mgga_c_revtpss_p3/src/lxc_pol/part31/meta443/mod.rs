//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1579;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1580;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta443<F: Float>(t11860: F, t19501: F, t3117: F, t19611: F, t3095: F, t3092: F, t19414: F, t247: F, t3116: F, t1651: F, t4866: F, t1045: F, t2857: F, t4181: F, t2852: F, t11703: F, t4910: F, t11859: F, t15850: F, t16095: F, t16165: F, t16218: F, t16220: F, t1675: F, t3091: F, t3115: F, t4837: F, t11264: F, t11675: F, t11818: F, t11875: F, t11927: F, t15583: F, t15618: F, t15662: F, t15707: F, t15862: F, t15865: F, t15892: F, t15926: F, t15942: F, t19622: F, t19626: F, t19636: F, t19641: F, t19645: F, t19685: F, t19729: F, t19763: F, t19797: F, t19813: F, t19841: F, t19885: F, t19895: F, t19901: F, t19923: F, t19950: F, t19989: F, t20012: F, t20036: F, t20073: F, t3127: F, t3241: F, t4783: F, t4825: F, t4899: F, t4907: F, t6268: F, t6285: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20075, t20079, t20083, t20089, t20090) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1579::<F>(t11860, t19501, t3117, t19611, t3095, t3092, t19414, t247, t3116, t1651, t4866, t1045);
        let (t20091, t20096, t20101, t20105, t20108) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1580::<F>(t20090, t3117, t1651, t2857, t4181, t3092, t2852, t11703, t19611, t4910, t11859, t15850, t16095, t16165, t16218, t16220, t1675, t20075, t20079, t20083, t3091, t3115, t4837);
        let t20112 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1581::<F>(t11264, t11675, t11818, t11859, t11875, t11927, t15583, t15618, t15662, t15707, t15862, t15865, t15892, t15926, t15942, t19622, t19626, t19636, t19641, t19645, t19685, t19729, t19763, t19797, t19813, t19841, t19885, t19895, t19901, t19923, t19950, t19989, t20012, t20036, t20073, t20108, t3091, t3127, t3241, t4783, t4825, t4899, t4907, t6268, t6285);
    (t20075, t20079, t20083, t20089, t20091, t20096, t20101, t20105, t20112)
}
