//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1006 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1006(t11524: f64, t19467: f64, t981: f64, t15538: f64, t4719: f64, t15542: f64, t41224: f64, t6189: f64, t19147: f64, t3022: f64, t18900: f64, t3333: f64, t41937: f64, t5023: f64, t6400: f64, t64335: f64, t64338: f64, t64340: f64, t64342: f64, t64344: f64, t64346: f64, t64404: f64, t64465: f64, t379: f64, t4746: f64, t1679: f64, t3057: f64, t1078: f64, t6244: f64, t1079: f64, t11214: f64, t11224: f64, t15578: f64, t16254: f64, t16305: f64, t16312: f64, t16313: f64, t16314: f64, t16603: f64, t16605: f64, t19400: f64, t20204: f64, t3066: f64, t3076: f64, t3268: f64, t3325: f64, t4764: f64, t4941: f64, t53108: f64, t53174: f64, t56087: f64, t6258: f64, t6259: f64, t6392: f64, t995: f64, t11187: f64, t11220: f64, t15579: f64, t16249: f64, t16318: f64, t16592: f64, t19351: f64, t19385: f64, t19415: f64, t19421: f64, t20191: f64, t20215: f64, t3047: f64, t3063: f64, t3067: f64, t3271: f64, t4747: f64, t4752: f64, t4778: f64, t4935: f64, t6393: f64, t1678: f64, t4743: f64, t11120: f64, t1651: f64, t1097: f64, t11210: f64, t15886: f64, t16287: f64, t16321: f64, t16322: f64, t16327: f64, t16591: f64, t16604: f64, t1680: f64, t1696: f64, t19429: f64, t3058: f64, t3059: f64, t53027: f64, t55416: f64, t1071: f64, t19462: f64, t19856: f64, t378: f64, t1647: f64, t4930: f64, t1000: f64, t11128: f64, t11195: f64, t16317: f64, t16374: f64, t1652: f64, t19428: f64, t20168: f64, t3043: f64, t42060: f64, t4772: f64, t5015: f64, t53208: f64, t6345: f64, t6351: f64, t989: f64, t996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64521, t64523, t64527, t64529, t64531, t64532) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3438(t11524, t19467, t981, t15538, t4719, t15542, t41224, t6189, t19147, t3022, t18900, t3333, t41937, t5023, t6400, t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64465);
        let t64567 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439(t379, t4746, t1679, t3057, t1078, t6244, t1079, t11214, t11224, t15578, t16254, t16305, t16312, t16313, t16314, t16603, t16605, t19400, t20204, t3066, t3076, t3268, t3325, t4764, t4941, t53108, t53174, t56087, t6258, t6259, t6392, t995);
        let t64592 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440(t11187, t11220, t11224, t15579, t16249, t16318, t16592, t19351, t19385, t19400, t19415, t19421, t20191, t20215, t3047, t3063, t3067, t3271, t4747, t4752, t4778, t4935, t6393);
        let t64626 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441(t1678, t4743, t11120, t1651, t1079, t1097, t11210, t15886, t16287, t16321, t16322, t16327, t16591, t16592, t16603, t16604, t1680, t1696, t19429, t3058, t3059, t4752, t4778, t4935, t53027, t55416, t6392, t6393, t995);
        let (t64647, t64661) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442(t1071, t19462, t19856, t378, t1647, t4930, t3059, t6244, t1000, t1079, t1097, t11128, t11195, t16317, t16374, t1652, t16603, t19428, t20168, t20215, t3043, t3047, t42060, t4772, t4941, t5015, t53208, t6259, t6345, t6351, t989, t995, t996);
    (t64521, t64523, t64527, t64529, t64531, t64532, t64567, t64592, t64626, t64647, t64661)
}
