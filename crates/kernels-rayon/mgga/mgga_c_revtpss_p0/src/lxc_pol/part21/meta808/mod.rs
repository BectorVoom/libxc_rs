//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta808 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2951;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta808(t15905: f64, t43420: f64, t43574: f64, t11922: f64, t15781: f64, t4892: f64, t42865: f64, t72: f64, t3088: f64, t43472: f64, t1668: f64, t42871: f64, t43401: f64, t11247: f64, t15758: f64, t15787: f64, t15910: f64, t15957: f64, t15963: f64, t16084: f64, t3091: f64, t3092: f64, t3117: f64, t3154: f64, t357: f64, t42369: f64, t42374: f64, t42377: f64, t42383: f64, t11620: f64, t1651: f64, t11710: f64, t15969: f64, t1062: f64, t15655: f64, t11239: f64, t1647: f64, t11245: f64, t11255: f64, t11643: f64, t15707: f64, t1042: f64, t1045: f64, t1063: f64, t11202: f64, t11252: f64, t11259: f64, t11933: f64, t1469: f64, t15716: f64, t16045: f64, t3115: f64, t3130: f64, t42421: f64, t42439: f64, t4872: f64, t51963: f64, t53474: f64, t15711: f64, t3106: f64, t15935: f64, t372: f64, t15936: f64, t4786: f64, t3151: f64, t3162: f64, t606: f64, t15904: f64, t245: f64, t12167: f64, t11632: f64, t11653: f64, t11788: f64, t15689: f64, t15691: f64, t15700: f64, t16104: f64, t16222: f64, t16226: f64, t19878: f64, t3105: f64, t3133: f64, t3155: f64, t42155: f64, t42450: f64, t42454: f64, t4839: f64, t53450: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53654, t53657, t53661, t53667, t53668, t53669, t53670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947(t15905, t43420, t43574, t11922, t15781, t4892, t42865, t72, t3088, t43472, t1668, t42871);
        let t53682 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948(t43401, t53668, t11247, t15758, t15787, t15910, t15957, t15963, t16084, t3091, t3092, t3117, t3154, t357, t42369, t42374, t42377, t42383, t53654, t53657, t53661, t53669, t53670);
        let (t53683, t53690, t53692, t53703, t53704, t53707, t53710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949(t11620, t1651, t11710, t15969, t4892, t1062, t15655, t11239, t1647, t11245, t11255, t11643, t15707);
        let t53716 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950(t1042, t1045, t1063, t11202, t11252, t11259, t11933, t1469, t15716, t16045, t3115, t3117, t3130, t42421, t42439, t4872, t51963, t53474, t53683, t53690, t53692, t53704, t53707, t53710);
        let (t53724, t53728, t53729, t53735, t53739) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2951(t15711, t3106, t15935, t372, t15936, t4786, t1469, t3151, t3162, t606, t15904, t245);
        let (t53740, t53759) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952(t3088, t53739, t12167, t1042, t1063, t11632, t11653, t11788, t15689, t15691, t15700, t15935, t16104, t16222, t16226, t19878, t3105, t3133, t3151, t3155, t42155, t42450, t42454, t4839, t53450, t53724, t53728, t53729, t53735, t606, t905);
    (t53667, t53668, t53670, t53682, t53683, t53703, t53716, t53729, t53735, t53739, t53740, t53759)
}
