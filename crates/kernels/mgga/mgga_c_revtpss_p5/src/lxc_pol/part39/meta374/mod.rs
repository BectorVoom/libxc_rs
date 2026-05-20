//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1320;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1321;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1322;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1323;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1324;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1325;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta374<F: Float>(t1469: F, t2251: F, t15935: F, t1042: F, t3173: F, t4879: F, t1063: F, t11802: F, t11814: F, t11818: F, t11994: F, t15917: F, t15922: F, t15926: F, t15932: F, t3115: F, t3120: F, t3164: F, t3188: F, t4803: F, t4808: F, t4825: F, t4902: F, t4186: F, t999: F, t4872: F, t4866: F, t73: F, t3095: F, t3092: F, t2857: F, t357: F, t4781: F, t11659: F, t3154: F, t1592: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t11672: F, t11675: F, t11881: F, t11886: F, t12004: F, t1675: F, t3127: F, t4783: F, t4892: F, t4899: F, t3252: F, t4574: F, t15145: F, t4915: F, t15149: F, t15154: F, t4919: F, t15130: F, t15135: F, t1012: F, t11821: F, t15140: F, t15780: F, t4900: F, t3117: F, t3133: F, t4893: F, t3059: F, t11927: F, t11933: F, t4907: F, t4912: F, t11922: F, t4906: F, t4910: F, t3075: F, t11670: F, t4890: F, t3317: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t15936 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1319::<F>(t1469, t2251);
        let t15949 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1320::<F>(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
        let (t15952, t15957, t15959, t15964) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1321::<F>(t4186, t999, t4872, t1042, t4866, t73, t3095, t3092, t2857, t357, t2251, t4781);
        let (t15965, t15970, t15975, t15986, t15988) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1322::<F>(t15964, t3092, t11659, t3154, t1592, t357, t11710, t4782, t3091, t1014, t140, t4579);
        let t15991 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1323::<F>(t1011, t15988, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let (t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1324::<F>(t140, t3252, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
        let t16034 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1325::<F>(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
        let (t16037, t16040, t16045, t16048, t16049) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1326::<F>(t11922, t4906, t3115, t15957, t4910, t3117, t3075, t357, t4781, t11670, t4890, t3317);
    (t15936, t15949, t15957, t15991, t16034, t16037, t16040, t16045, t16048, t16049)
}
