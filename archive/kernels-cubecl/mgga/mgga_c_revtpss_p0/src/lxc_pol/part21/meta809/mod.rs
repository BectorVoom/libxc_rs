//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta809 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2954;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2957;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta809<F: Float>(t1063: F, t1592: F, t247: F, t42778: F, t11922: F, t16044: F, t3115: F, t11714: F, t11866: F, t15716: F, t15847: F, t16078: F, t16201: F, t16205: F, t16210: F, t3106: F, t3116: F, t42477: F, t42481: F, t4808: F, t53089: F, t11994: F, t15769: F, t3151: F, t4772: F, t3298: F, t4746: F, t4891: F, t11744: F, t4834: F, t12012: F, t15822: F, t12009: F, t15823: F, t11862: F, t11875: F, t11991: F, t12017: F, t15926: F, t3117: F, t3157: F, t3162: F, t42391: F, t42487: F, t42496: F, t4803: F, t4875: F, t11710: F, t16089: F, t16090: F, t3059: F, t606: F, t11883: F, t4924: F, t2258: F, t999: F, t11703: F, t15584: F, t15968: F, t16095: F, t1656: F, t3092: F, t42499: F, t42506: F, t42516: F, t42537: F, t42721: F, t43082: F, t4573: F, t4578: F, t4873: F, t13396: F, t4786: F, t1086: F, t15654: F, t3090: F, t16077: F, t225: F, t53222: F, t366: F, t1025: F, t371: F, t4852: F, t676: F, t53014: F, t11656: F, t15734: F, t1028: F, t11811: F, t11944: F, t15129: F, t15656: F, t15700: F, t15973: F, t16096: F, t16222: F, t3120: F, t3220: F, t42328: F, t4858: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t53785 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2953::<F>(t1063, t1592, t247, t42778, t11922, t16044, t3115, t11714, t11866, t15716, t15847, t16078, t16201, t16205, t16210, t3106, t3116, t42477, t42481, t4808, t53089);
        let (t53790, t53792, t53800, t53805, t53807, t53810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2954::<F>(t11994, t15769, t3151, t4772, t3298, t4746, t4891, t11744, t4834, t12012, t15822, t12009, t15823);
        let t53816 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955::<F>(t11862, t11875, t11991, t12017, t15926, t3117, t3157, t3162, t42391, t42487, t42496, t4803, t4875, t53790, t53792, t53800, t53805, t53807, t53810);
        let (t53822, t53835, t53844) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2956::<F>(t11710, t16089, t16090, t3059, t606, t11883, t4924, t2258, t999, t11703, t11991, t15584, t15968, t16095, t1656, t3092, t42499, t42506, t42516, t42537, t42721, t43082, t4573, t4578, t4808, t4873);
        let (t53846, t53855, t53859, t53865, t53866, t53875) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2957::<F>(t13396, t4786, t1086, t15654, t3090, t11922, t16077, t3115, t225, t53222, t366, t1025, t371, t4852, t676);
        let (t53877, t53883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2958::<F>(t53875, t225, t53014, t366, t11656, t15734, t1028, t11703, t11811, t11944, t15129, t15584, t15656, t15700, t15973, t16095, t16096, t16222, t3120, t3220, t42328, t4858, t4873, t53846, t53855, t53859, t53866);
    (t53785, t53792, t53816, t53822, t53835, t53844, t53846, t53865, t53877, t53883)
}
