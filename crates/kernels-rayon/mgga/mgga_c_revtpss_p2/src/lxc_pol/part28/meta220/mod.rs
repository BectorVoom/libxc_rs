//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1042;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1043;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1044;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1045;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta220(t1012: f64, t1014: f64, t4579: f64, t3252: f64, t4574: f64, t140: f64, t1655: f64, t1011: f64, t1656: f64, t3115: f64, t3234: f64, t3241: f64, t3245: f64, t4887: f64, t4892: f64, t4896: f64, t4899: f64, t4902: f64, t4907: f64, t4912: f64, t1063: f64, t1671: f64, t3082: f64, t3086: f64, t3091: f64, t3169: f64, t375: f64, t4783: f64, t4788: f64, t4792: f64, t4794: f64, t4798: f64, t4803: f64, t4808: f64, t4848: f64, t4883: f64, t225: f64, t385: f64, t1678: f64, t342: f64, t1695: f64, t999: f64, t1079: f64, t1096: f64, t3269: f64, t1086: f64, t1647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4915, t4916, t4919, t4920, t4924, t4928) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1042(t1012, t1014, t4579, t3252, t4574, t140, t1655, t1011, t1656, t3115, t3234, t3241, t3245, t4887, t4892, t4896, t4899, t4902, t4907, t4912);
        let t4930 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1043(t1063, t1671, t3082, t3086, t3091, t3169, t375, t4783, t4788, t4792, t4794, t4798, t4803, t4808, t4848, t4883, t4928);
        let (t4932, t4935, t4941) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1044(t225, t385, t4930, t1678, t342, t1695, t999, t1079);
        let (t4946, t4947) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1045(t1096, t1695, t3269);
        let t4954 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1046(t1086, t1647);
    (t4915, t4916, t4919, t4920, t4924, t4930, t4932, t4935, t4941, t4946, t4947, t4954)
}
