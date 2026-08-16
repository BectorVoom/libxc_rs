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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1082(t14230: f64, t2782: f64, t48083: f64, t4086: f64, t543: f64, t74922: f64, t10073: f64, t22365: f64, t14141: f64, t14143: f64, t5658: f64, t676: f64, t13921: f64, t14193: f64, t22005: f64, t46416: f64, t46443: f64, t46448: f64, t46452: f64, t47976: f64, t47978: f64, t47980: f64, t47985: f64, t5767: f64, t820: f64, t22252: f64, t555: f64, t1419: f64, t6843: f64, t14224: f64, t14238: f64, t6861: f64, t10130: f64, t1399: f64, t46463: f64, t47995: f64, t47999: f64, t48003: f64, t48005: f64, t48008: f64, t48013: f64, t48020: f64, t49376: f64, t5735: f64, t5745: f64, t5755: f64, t6874: f64, t22373: f64, t10069: f64, t22369: f64, t14216: f64, t14239: f64, t14220: f64, t48007: f64, t46465: f64, t46490: f64, t48022: f64, t48027: f64, t48029: f64, t48036: f64, t48039: f64, t48041: f64, t6844: f64, t1883: f64, t4100: f64, t73842: f64, t22331: f64, t2470: f64, t4101: f64, t48048: f64, t5741: f64, t10090: f64, t122: f64, t14144: f64, t2482: f64, t72: f64, t9994: f64, t14145: f64, t4114: f64, t10014: f64, t22336: f64, t46496: f64, t46500: f64, t46505: f64, t48049: f64, t48058: f64, t48066: f64, t1398: f64, t73820: f64, t47371: f64, t6862: f64, t10022: f64, t22315: f64, t46457: f64, t136: f64, t2457: f64, t47429: f64, t22016: f64, t46510: f64, t46515: f64, t46518: f64, t48076: f64, t48079: f64, t48081: f64, t48085: f64, t48089: f64, t22332: f64, t22351: f64, t2439: f64, t2777: f64, t22253: f64, t686: f64, t22335: f64, t14122: f64, t22321: f64, t4057: f64, t46520: f64, t46526: f64, t49167: f64, t49172: f64, t49176: f64, t49178: f64, t49186: f64, t49189: f64, t5659: f64, t22361: f64, t10139: f64, t14255: f64, t46536: f64, t46542: f64, t49198: f64, t49200: f64, t49203: f64, t49208: f64, t49210: f64, t5675: f64, t14171: f64, t1882: f64, t13805: f64, t22009: f64, t46563: f64, t46570: f64, t46572: f64, t49238: f64, t49242: f64, t22307: f64, t545: f64, t689: f64, t869: f64, t14242: f64, t10023: f64, t22314: f64, t4004: f64, t47348: f64, t49248: f64, t49252: f64, t49256: f64, t49260: f64, t49263: f64, t49273: f64, t9840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74935, t74943, t74945, t74949) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3901(t14230, t2782, t48083, t4086, t543, t74922, t10073, t22365, t14141, t14143, t5658, t676);
        let t74954 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3902(t13921, t14193, t22005, t46416, t46443, t46448, t46452, t47976, t47978, t47980, t47985, t5767, t74935, t74943, t74945, t74949, t820);
        let (t74965, t74973, t74979, t74982) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3903(t22252, t555, t1419, t6843, t14224, t14238, t2782, t6861);
        let t74987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3904(t2782, t4086, t543, t74982, t10130, t1399, t46463, t47995, t47999, t48003, t48005, t48008, t48013, t48020, t49376, t5735, t5745, t5755, t6874, t74965, t74973, t74979, t820);
        let t75009 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3905(t10073, t22373, t10069, t22369, t14216, t14239, t14220, t48007, t10130, t46465, t46490, t48022, t48027, t48029, t48036, t48039, t48041, t6844, t820);
        let (t75014, t75018, t75021, t75024, t75026) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3906(t1883, t5658, t2782, t4100, t543, t73842, t22331, t2470, t4101, t48048, t5741, t10073, t22369);
        let t75044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3907(t10090, t122, t14144, t2482, t6861, t72, t9994, t14145, t4114, t10014, t22336, t46496, t46500, t46505, t48049, t48058, t48066, t75014, t75018, t75021, t75024, t75026);
        let t75070 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3908(t1398, t73820, t2782, t47371, t6862, t10022, t22315, t46457, t136, t2457, t47429, t14193, t22016, t46510, t46515, t46518, t48076, t48079, t48081, t48085, t48089, t5658, t5735);
        let t75097 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3909(t10014, t22332, t22351, t2439, t2777, t22253, t4101, t686, t72, t22335, t2470, t14122, t22321, t4057, t46520, t46526, t49167, t49172, t49176, t49178, t49186, t49189, t5659, t5755, t820);
        let t75125 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3910(t10073, t22361, t10069, t22373, t10139, t136, t2457, t6874, t1399, t14255, t46536, t46542, t49198, t49200, t49203, t49208, t49210, t5659, t5675, t5745, t5755, t74965, t74982, t820);
        let t75155 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3911(t10139, t136, t2457, t6844, t14145, t14171, t1882, t2482, t10069, t22361, t22365, t13805, t14193, t22005, t22009, t4057, t46563, t46570, t46572, t49238, t49242, t5675, t5745, t5755, t74973, t74982);
        let t75182 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3912(t22307, t545, t689, t869, t14239, t14242, t10023, t22314, t2470, t22009, t4004, t4057, t47348, t49248, t49252, t49256, t49260, t49263, t49273, t5745, t5755, t9840);
    (t74954, t74965, t74973, t74982, t74987, t75009, t75044, t75070, t75097, t75125, t75155, t75182)
}
