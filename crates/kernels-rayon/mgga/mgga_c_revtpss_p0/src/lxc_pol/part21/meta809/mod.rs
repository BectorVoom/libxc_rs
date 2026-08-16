//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta809 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2954;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2957;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta809(t1063: f64, t1592: f64, t247: f64, t42778: f64, t11922: f64, t16044: f64, t3115: f64, t11714: f64, t11866: f64, t15716: f64, t15847: f64, t16078: f64, t16201: f64, t16205: f64, t16210: f64, t3106: f64, t3116: f64, t42477: f64, t42481: f64, t4808: f64, t53089: f64, t11994: f64, t15769: f64, t3151: f64, t4772: f64, t3298: f64, t4746: f64, t4891: f64, t11744: f64, t4834: f64, t12012: f64, t15822: f64, t12009: f64, t15823: f64, t11862: f64, t11875: f64, t11991: f64, t12017: f64, t15926: f64, t3117: f64, t3157: f64, t3162: f64, t42391: f64, t42487: f64, t42496: f64, t4803: f64, t4875: f64, t11710: f64, t16089: f64, t16090: f64, t3059: f64, t606: f64, t11883: f64, t4924: f64, t2258: f64, t999: f64, t11703: f64, t15584: f64, t15968: f64, t16095: f64, t1656: f64, t3092: f64, t42499: f64, t42506: f64, t42516: f64, t42537: f64, t42721: f64, t43082: f64, t4573: f64, t4578: f64, t4873: f64, t13396: f64, t4786: f64, t1086: f64, t15654: f64, t3090: f64, t16077: f64, t225: f64, t53222: f64, t366: f64, t1025: f64, t371: f64, t4852: f64, t676: f64, t53014: f64, t11656: f64, t15734: f64, t1028: f64, t11811: f64, t11944: f64, t15129: f64, t15656: f64, t15700: f64, t15973: f64, t16096: f64, t16222: f64, t3120: f64, t3220: f64, t42328: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t53785 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953(t1063, t1592, t247, t42778, t11922, t16044, t3115, t11714, t11866, t15716, t15847, t16078, t16201, t16205, t16210, t3106, t3116, t42477, t42481, t4808, t53089);
        let (t53790, t53792, t53800, t53805, t53807, t53810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2954(t11994, t15769, t3151, t4772, t3298, t4746, t4891, t11744, t4834, t12012, t15822, t12009, t15823);
        let t53816 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955(t11862, t11875, t11991, t12017, t15926, t3117, t3157, t3162, t42391, t42487, t42496, t4803, t4875, t53790, t53792, t53800, t53805, t53807, t53810);
        let (t53822, t53835, t53844) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956(t11710, t16089, t16090, t3059, t606, t11883, t4924, t2258, t999, t11703, t11991, t15584, t15968, t16095, t1656, t3092, t42499, t42506, t42516, t42537, t42721, t43082, t4573, t4578, t4808, t4873);
        let (t53846, t53855, t53859, t53865, t53866, t53875) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2957(t13396, t4786, t1086, t15654, t3090, t11922, t16077, t3115, t225, t53222, t366, t1025, t371, t4852, t676);
        let (t53877, t53883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958(t53875, t225, t53014, t366, t11656, t15734, t1028, t11703, t11811, t11944, t15129, t15584, t15656, t15700, t15973, t16095, t16096, t16222, t3120, t3220, t42328, t4858, t4873, t53846, t53855, t53859, t53866);
    (t53785, t53792, t53816, t53822, t53835, t53844, t53846, t53865, t53877, t53883)
}
