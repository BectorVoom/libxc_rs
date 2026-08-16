//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1082 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3901;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3902;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3903;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3904;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3905;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3906;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3907;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3908;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3909;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3910;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3911;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1082<F: Float>(t14230: F, t2782: F, t48083: F, t4086: F, t543: F, t74922: F, t10073: F, t22365: F, t14141: F, t14143: F, t5658: F, t676: F, t13921: F, t14193: F, t22005: F, t46416: F, t46443: F, t46448: F, t46452: F, t47976: F, t47978: F, t47980: F, t47985: F, t5767: F, t820: F, t22252: F, t555: F, t1419: F, t6843: F, t14224: F, t14238: F, t6861: F, t10130: F, t1399: F, t46463: F, t47995: F, t47999: F, t48003: F, t48005: F, t48008: F, t48013: F, t48020: F, t49376: F, t5735: F, t5745: F, t5755: F, t6874: F, t22373: F, t10069: F, t22369: F, t14216: F, t14239: F, t14220: F, t48007: F, t46465: F, t46490: F, t48022: F, t48027: F, t48029: F, t48036: F, t48039: F, t48041: F, t6844: F, t1883: F, t4100: F, t73842: F, t22331: F, t2470: F, t4101: F, t48048: F, t5741: F, t10090: F, t122: F, t14144: F, t2482: F, t72: F, t9994: F, t14145: F, t4114: F, t10014: F, t22336: F, t46496: F, t46500: F, t46505: F, t48049: F, t48058: F, t48066: F, t1398: F, t73820: F, t47371: F, t6862: F, t10022: F, t22315: F, t46457: F, t136: F, t2457: F, t47429: F, t22016: F, t46510: F, t46515: F, t46518: F, t48076: F, t48079: F, t48081: F, t48085: F, t48089: F, t22332: F, t22351: F, t2439: F, t2777: F, t22253: F, t686: F, t22335: F, t14122: F, t22321: F, t4057: F, t46520: F, t46526: F, t49167: F, t49172: F, t49176: F, t49178: F, t49186: F, t49189: F, t5659: F, t22361: F, t10139: F, t14255: F, t46536: F, t46542: F, t49198: F, t49200: F, t49203: F, t49208: F, t49210: F, t5675: F, t14171: F, t1882: F, t13805: F, t22009: F, t46563: F, t46570: F, t46572: F, t49238: F, t49242: F, t22307: F, t545: F, t689: F, t869: F, t14242: F, t10023: F, t22314: F, t4004: F, t47348: F, t49248: F, t49252: F, t49256: F, t49260: F, t49263: F, t49273: F, t9840: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74935, t74943, t74945, t74949) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3901::<F>(t14230, t2782, t48083, t4086, t543, t74922, t10073, t22365, t14141, t14143, t5658, t676);
        let t74954 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3902::<F>(t13921, t14193, t22005, t46416, t46443, t46448, t46452, t47976, t47978, t47980, t47985, t5767, t74935, t74943, t74945, t74949, t820);
        let (t74965, t74973, t74979, t74982) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3903::<F>(t22252, t555, t1419, t6843, t14224, t14238, t2782, t6861);
        let t74987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3904::<F>(t2782, t4086, t543, t74982, t10130, t1399, t46463, t47995, t47999, t48003, t48005, t48008, t48013, t48020, t49376, t5735, t5745, t5755, t6874, t74965, t74973, t74979, t820);
        let t75009 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3905::<F>(t10073, t22373, t10069, t22369, t14216, t14239, t14220, t48007, t10130, t46465, t46490, t48022, t48027, t48029, t48036, t48039, t48041, t6844, t820);
        let (t75014, t75018, t75021, t75024, t75026) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3906::<F>(t1883, t5658, t2782, t4100, t543, t73842, t22331, t2470, t4101, t48048, t5741, t10073, t22369);
        let t75044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3907::<F>(t10090, t122, t14144, t2482, t6861, t72, t9994, t14145, t4114, t10014, t22336, t46496, t46500, t46505, t48049, t48058, t48066, t75014, t75018, t75021, t75024, t75026);
        let t75070 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3908::<F>(t1398, t73820, t2782, t47371, t6862, t10022, t22315, t46457, t136, t2457, t47429, t14193, t22016, t46510, t46515, t46518, t48076, t48079, t48081, t48085, t48089, t5658, t5735);
        let t75097 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3909::<F>(t10014, t22332, t22351, t2439, t2777, t22253, t4101, t686, t72, t22335, t2470, t14122, t22321, t4057, t46520, t46526, t49167, t49172, t49176, t49178, t49186, t49189, t5659, t5755, t820);
        let t75125 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3910::<F>(t10073, t22361, t10069, t22373, t10139, t136, t2457, t6874, t1399, t14255, t46536, t46542, t49198, t49200, t49203, t49208, t49210, t5659, t5675, t5745, t5755, t74965, t74982, t820);
        let t75155 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3911::<F>(t10139, t136, t2457, t6844, t14145, t14171, t1882, t2482, t10069, t22361, t22365, t13805, t14193, t22005, t22009, t4057, t46563, t46570, t46572, t49238, t49242, t5675, t5745, t5755, t74973, t74982);
        let t75182 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3912::<F>(t22307, t545, t689, t869, t14239, t14242, t10023, t22314, t2470, t22009, t4004, t4057, t47348, t49248, t49252, t49256, t49260, t49263, t49273, t5745, t5755, t9840);
    (t74954, t74965, t74973, t74982, t74987, t75009, t75044, t75070, t75097, t75125, t75155, t75182)
}
