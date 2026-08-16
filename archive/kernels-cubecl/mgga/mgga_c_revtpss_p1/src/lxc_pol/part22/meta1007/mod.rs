//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1007 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1007<F: Float>(t3056: F, t6234: F, t378: F, t1076: F, t11121: F, t11195: F, t16275: F, t16318: F, t16328: F, t19342: F, t20175: F, t20211: F, t3047: F, t3058: F, t3059: F, t3060: F, t3076: F, t3269: F, t3325: F, t3326: F, t4747: F, t4752: F, t4758: F, t53160: F, t53167: F, t55413: F, t6350: F, t6393: F, t15669: F, t379: F, t11190: F, t11224: F, t16314: F, t16371: F, t16597: F, t1696: F, t19381: F, t19396: F, t19415: F, t19425: F, t20172: F, t3052: F, t3063: F, t3075: F, t4773: F, t4778: F, t4947: F, t53093: F, t6251: F, t995: F, t20112: F, t994: F, t1000: F, t1079: F, t11187: F, t15579: F, t16254: F, t16312: F, t16322: F, t16374: F, t16603: F, t19421: F, t19428: F, t20195: F, t3264: F, t4743: F, t4764: F, t4932: F, t4935: F, t4940: F, t53130: F, t6392: F, t4746: F, t4930: F, t6244: F, t11128: F, t11201: F, t15648: F, t16242: F, t16255: F, t16284: F, t16292: F, t16295: F, t16313: F, t16344: F, t16591: F, t16600: F, t1695: F, t19385: F, t19403: F, t52927: F, t55458: F, t996: F, t19855: F, t993: F, t16305: F, t16340: F, t16362: F, t3067: F, t3270: F, t3271: F, t5016: F, t6258: F, t6259: F) -> (F, F, F, F, F, F, F, F) {
        let (t64686, t64694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443::<F>(t3056, t6234, t378, t1076, t11121, t11195, t16275, t16318, t16328, t19342, t20175, t20211, t3047, t3058, t3059, t3060, t3076, t3269, t3325, t3326, t4747, t4752, t4758, t53160, t53167, t55413, t6350, t6393);
        let t64722 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444::<F>(t15669, t379, t11190, t11224, t16314, t16328, t16371, t16597, t1696, t19381, t19396, t19415, t19425, t20172, t3052, t3063, t3075, t3269, t4773, t4778, t4947, t53093, t6251, t6350, t995);
        let t64753 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3445::<F>(t20112, t994, t1000, t1079, t11187, t15579, t16254, t16312, t16322, t16374, t16597, t16603, t19421, t19428, t20172, t20195, t3052, t3075, t3264, t4743, t4747, t4764, t4932, t4935, t4940, t53130, t6392, t995);
        let (t64772, t64788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3446::<F>(t4746, t4930, t3075, t6244, t1000, t1076, t1079, t11128, t11201, t15648, t16242, t16255, t16284, t16292, t16295, t16312, t16313, t16344, t16591, t16600, t1695, t19385, t19403, t3063, t3269, t4935, t52927, t55458, t6251, t995, t996);
        let (t64816, t64822) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447::<F>(t19855, t993, t378, t1000, t11190, t16305, t16340, t16362, t16374, t19425, t20175, t20195, t20211, t3058, t3067, t3264, t3269, t3270, t3271, t4773, t5016, t6244, t6258, t6259, t995);
    (t64686, t64694, t64722, t64753, t64772, t64788, t64816, t64822)
}
