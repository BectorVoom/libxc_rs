//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1007 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1007(t3056: f64, t6234: f64, t378: f64, t1076: f64, t11121: f64, t11195: f64, t16275: f64, t16318: f64, t16328: f64, t19342: f64, t20175: f64, t20211: f64, t3047: f64, t3058: f64, t3059: f64, t3060: f64, t3076: f64, t3269: f64, t3325: f64, t3326: f64, t4747: f64, t4752: f64, t4758: f64, t53160: f64, t53167: f64, t55413: f64, t6350: f64, t6393: f64, t15669: f64, t379: f64, t11190: f64, t11224: f64, t16314: f64, t16371: f64, t16597: f64, t1696: f64, t19381: f64, t19396: f64, t19415: f64, t19425: f64, t20172: f64, t3052: f64, t3063: f64, t3075: f64, t4773: f64, t4778: f64, t4947: f64, t53093: f64, t6251: f64, t995: f64, t20112: f64, t994: f64, t1000: f64, t1079: f64, t11187: f64, t15579: f64, t16254: f64, t16312: f64, t16322: f64, t16374: f64, t16603: f64, t19421: f64, t19428: f64, t20195: f64, t3264: f64, t4743: f64, t4764: f64, t4932: f64, t4935: f64, t4940: f64, t53130: f64, t6392: f64, t4746: f64, t4930: f64, t6244: f64, t11128: f64, t11201: f64, t15648: f64, t16242: f64, t16255: f64, t16284: f64, t16292: f64, t16295: f64, t16313: f64, t16344: f64, t16591: f64, t16600: f64, t1695: f64, t19385: f64, t19403: f64, t52927: f64, t55458: f64, t996: f64, t19855: f64, t993: f64, t16305: f64, t16340: f64, t16362: f64, t3067: f64, t3270: f64, t3271: f64, t5016: f64, t6258: f64, t6259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64686, t64694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443(t3056, t6234, t378, t1076, t11121, t11195, t16275, t16318, t16328, t19342, t20175, t20211, t3047, t3058, t3059, t3060, t3076, t3269, t3325, t3326, t4747, t4752, t4758, t53160, t53167, t55413, t6350, t6393);
        let t64722 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444(t15669, t379, t11190, t11224, t16314, t16328, t16371, t16597, t1696, t19381, t19396, t19415, t19425, t20172, t3052, t3063, t3075, t3269, t4773, t4778, t4947, t53093, t6251, t6350, t995);
        let t64753 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445(t20112, t994, t1000, t1079, t11187, t15579, t16254, t16312, t16322, t16374, t16597, t16603, t19421, t19428, t20172, t20195, t3052, t3075, t3264, t4743, t4747, t4764, t4932, t4935, t4940, t53130, t6392, t995);
        let (t64772, t64788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446(t4746, t4930, t3075, t6244, t1000, t1076, t1079, t11128, t11201, t15648, t16242, t16255, t16284, t16292, t16295, t16312, t16313, t16344, t16591, t16600, t1695, t19385, t19403, t3063, t3269, t4935, t52927, t55458, t6251, t995, t996);
        let (t64816, t64822) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447(t19855, t993, t378, t1000, t11190, t16305, t16340, t16362, t16374, t19425, t20175, t20195, t20211, t3058, t3067, t3264, t3269, t3270, t3271, t4773, t5016, t6244, t6258, t6259, t995);
    (t64686, t64694, t64722, t64753, t64772, t64788, t64816, t64822)
}
