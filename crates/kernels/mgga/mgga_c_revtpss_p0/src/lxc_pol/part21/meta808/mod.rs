//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta808 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2951;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta808<F: Float>(t15905: F, t43420: F, t43574: F, t11922: F, t15781: F, t4892: F, t42865: F, t72: F, t3088: F, t43472: F, t1668: F, t42871: F, t43401: F, t11247: F, t15758: F, t15787: F, t15910: F, t15957: F, t15963: F, t16084: F, t3091: F, t3092: F, t3117: F, t3154: F, t357: F, t42369: F, t42374: F, t42377: F, t42383: F, t11620: F, t1651: F, t11710: F, t15969: F, t1062: F, t15655: F, t11239: F, t1647: F, t11245: F, t11255: F, t11643: F, t15707: F, t1042: F, t1045: F, t1063: F, t11202: F, t11252: F, t11259: F, t11933: F, t1469: F, t15716: F, t16045: F, t3115: F, t3130: F, t42421: F, t42439: F, t4872: F, t51963: F, t53474: F, t15711: F, t3106: F, t15935: F, t372: F, t15936: F, t4786: F, t3151: F, t3162: F, t606: F, t15904: F, t245: F, t12167: F, t11632: F, t11653: F, t11788: F, t15689: F, t15691: F, t15700: F, t16104: F, t16222: F, t16226: F, t19878: F, t3105: F, t3133: F, t3155: F, t42155: F, t42450: F, t42454: F, t4839: F, t53450: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53654, t53657, t53661, t53667, t53668, t53669, t53670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947::<F>(t15905, t43420, t43574, t11922, t15781, t4892, t42865, t72, t3088, t43472, t1668, t42871);
        let t53682 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2948::<F>(t43401, t53668, t11247, t15758, t15787, t15910, t15957, t15963, t16084, t3091, t3092, t3117, t3154, t357, t42369, t42374, t42377, t42383, t53654, t53657, t53661, t53669, t53670);
        let (t53683, t53690, t53692, t53703, t53704, t53707, t53710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949::<F>(t11620, t1651, t11710, t15969, t4892, t1062, t15655, t11239, t1647, t11245, t11255, t11643, t15707);
        let t53716 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950::<F>(t1042, t1045, t1063, t11202, t11252, t11259, t11933, t1469, t15716, t16045, t3115, t3117, t3130, t42421, t42439, t4872, t51963, t53474, t53683, t53690, t53692, t53704, t53707, t53710);
        let (t53724, t53728, t53729, t53735, t53739) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2951::<F>(t15711, t3106, t15935, t372, t15936, t4786, t1469, t3151, t3162, t606, t15904, t245);
        let (t53740, t53759) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952::<F>(t3088, t53739, t12167, t1042, t1063, t11632, t11653, t11788, t15689, t15691, t15700, t15935, t16104, t16222, t16226, t19878, t3105, t3133, t3151, t3155, t42155, t42450, t42454, t4839, t53450, t53724, t53728, t53729, t53735, t606, t905);
    (t53667, t53668, t53670, t53682, t53683, t53703, t53716, t53729, t53735, t53739, t53740, t53759)
}
