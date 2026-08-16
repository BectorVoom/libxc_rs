//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1322;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1323;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1324;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1325;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1326;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1327;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1328;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta374(t1469: f64, t2251: f64, t15935: f64, t1042: f64, t3173: f64, t4879: f64, t1063: f64, t11802: f64, t11814: f64, t11818: f64, t11994: f64, t15917: f64, t15922: f64, t15926: f64, t15932: f64, t3115: f64, t3120: f64, t3164: f64, t3188: f64, t4803: f64, t4808: f64, t4825: f64, t4902: f64, t4186: f64, t999: f64, t4872: f64, t4866: f64, t73: f64, t3095: f64, t3092: f64, t2857: f64, t357: f64, t4781: f64, t11659: f64, t3154: f64, t1592: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t11672: f64, t11675: f64, t11881: f64, t11886: f64, t12004: f64, t1675: f64, t3127: f64, t4783: f64, t4892: f64, t4899: f64, t3252: f64, t4574: f64, t15145: f64, t4915: f64, t15149: f64, t15154: f64, t4919: f64, t15130: f64, t15135: f64, t1012: f64, t11821: f64, t15140: f64, t15780: f64, t4900: f64, t3117: f64, t3133: f64, t4893: f64, t3059: f64, t11927: f64, t11933: f64, t4907: f64, t4912: f64, t11922: f64, t4906: f64, t4910: f64, t3075: f64, t11670: f64, t4890: f64, t3317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t15936 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1322(t1469, t2251);
        let t15949 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1323(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
        let (t15952, t15957, t15959, t15964) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1324(t4186, t999, t4872, t1042, t4866, t73, t3095, t3092, t2857, t357, t2251, t4781);
        let (t15965, t15970, t15975, t15986, t15988) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1325(t15964, t3092, t11659, t3154, t1592, t357, t11710, t4782, t3091, t1014, t140, t4579);
        let t15991 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1326(t1011, t15988, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let (t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1327(t140, t3252, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
        let t16034 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1328(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
        let (t16037, t16040, t16045, t16048, t16049) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1329(t11922, t4906, t3115, t15957, t4910, t3117, t3075, t357, t4781, t11670, t4890, t3317);
    (t15936, t15949, t15957, t15991, t16034, t16037, t16040, t16045, t16048, t16049)
}
