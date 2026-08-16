//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1289;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1290;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1291;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1292;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1293;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1294;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta388(t19456: f64, t247: f64, t3116: f64, t3172: f64, t6311: f64, t3161: f64, t1043: f64, t6244: f64, t1045: f64, t3117: f64, t1668: f64, t4772: f64, t11866: f64, t11927: f64, t15716: f64, t15771: f64, t15774: f64, t15776: f64, t15817: f64, t1671: f64, t3115: f64, t4831: f64, t4834: f64, t4869: f64, t4879: f64, t6273: f64, t11134: f64, t11890: f64, t15189: f64, t15874: f64, t15875: f64, t15876: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t341: f64, t225: f64, t366: f64, t15696: f64, t4782: f64, t4787: f64, t1058: f64, t6318: f64, t1053: f64, t6317: f64, t4786: f64, t6096: f64, t3092: f64, t1062: f64, t15670: f64, t3109: f64, t1063: f64, t11672: f64, t11774: f64, t15796: f64, t15829: f64, t3091: f64, t375: f64, t4839: f64, t6268: f64, t19691: f64, t4801: f64, t1042: f64, t140: f64, t6284: f64, t1011: f64, t6288: f64, t6292: f64, t1015: f64, t18281: f64, t1012: f64, t6262: f64, t3127: f64, t11881: f64, t15986: f64, t15990: f64, t15996: f64, t16037: f64, t3241: f64, t6289: f64, t6293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19819, t19827, t19829, t19831, t19836) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1289(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
        let t19841 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1290(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1291(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19856, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1292(t19855, t341, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
        let t19885 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1293(t19872, t3092, t1062, t15670, t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t3091, t375, t4839, t6268);
        let (t19895, t19901, t19908, t19913, t19917, t19920) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1294(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let t19923 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1295(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
    (t19829, t19836, t19841, t19856, t19885, t19895, t19901, t19923)
}
