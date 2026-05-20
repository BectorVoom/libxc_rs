//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1006 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1006<F: Float>(t11524: F, t19467: F, t981: F, t15538: F, t4719: F, t15542: F, t41224: F, t6189: F, t19147: F, t3022: F, t18900: F, t3333: F, t41937: F, t5023: F, t6400: F, t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t64404: F, t64465: F, t379: F, t4746: F, t1679: F, t3057: F, t1078: F, t6244: F, t1079: F, t11214: F, t11224: F, t15578: F, t16254: F, t16305: F, t16312: F, t16313: F, t16314: F, t16603: F, t16605: F, t19400: F, t20204: F, t3066: F, t3076: F, t3268: F, t3325: F, t4764: F, t4941: F, t53108: F, t53174: F, t56087: F, t6258: F, t6259: F, t6392: F, t995: F, t11187: F, t11220: F, t15579: F, t16249: F, t16318: F, t16592: F, t19351: F, t19385: F, t19415: F, t19421: F, t20191: F, t20215: F, t3047: F, t3063: F, t3067: F, t3271: F, t4747: F, t4752: F, t4778: F, t4935: F, t6393: F, t1678: F, t4743: F, t11120: F, t1651: F, t1097: F, t11210: F, t15886: F, t16287: F, t16321: F, t16322: F, t16327: F, t16591: F, t16604: F, t1680: F, t1696: F, t19429: F, t3058: F, t3059: F, t53027: F, t55416: F, t1071: F, t19462: F, t19856: F, t378: F, t1647: F, t4930: F, t1000: F, t11128: F, t11195: F, t16317: F, t16374: F, t1652: F, t19428: F, t20168: F, t3043: F, t42060: F, t4772: F, t5015: F, t53208: F, t6345: F, t6351: F, t989: F, t996: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t64521, t64523, t64527, t64529, t64531, t64532) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438::<F>(t11524, t19467, t981, t15538, t4719, t15542, t41224, t6189, t19147, t3022, t18900, t3333, t41937, t5023, t6400, t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64465);
        let t64567 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439::<F>(t379, t4746, t1679, t3057, t1078, t6244, t1079, t11214, t11224, t15578, t16254, t16305, t16312, t16313, t16314, t16603, t16605, t19400, t20204, t3066, t3076, t3268, t3325, t4764, t4941, t53108, t53174, t56087, t6258, t6259, t6392, t995);
        let t64592 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440::<F>(t11187, t11220, t11224, t15579, t16249, t16318, t16592, t19351, t19385, t19400, t19415, t19421, t20191, t20215, t3047, t3063, t3067, t3271, t4747, t4752, t4778, t4935, t6393);
        let t64626 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441::<F>(t1678, t4743, t11120, t1651, t1079, t1097, t11210, t15886, t16287, t16321, t16322, t16327, t16591, t16592, t16603, t16604, t1680, t1696, t19429, t3058, t3059, t4752, t4778, t4935, t53027, t55416, t6392, t6393, t995);
        let (t64647, t64661) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442::<F>(t1071, t19462, t19856, t378, t1647, t4930, t3059, t6244, t1000, t1079, t1097, t11128, t11195, t16317, t16374, t1652, t16603, t19428, t20168, t20215, t3043, t3047, t42060, t4772, t4941, t5015, t53208, t6259, t6345, t6351, t989, t995, t996);
    (t64521, t64523, t64527, t64529, t64531, t64532, t64567, t64592, t64626, t64647, t64661)
}
