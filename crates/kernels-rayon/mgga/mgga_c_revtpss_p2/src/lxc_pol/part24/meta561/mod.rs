//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1687;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta561(t3011: f64, t3014: f64, t88351: f64, t981: f64, t6392: f64, t6244: f64, t6258: f64, t42013: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64, t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64, t341: f64, t1076: f64, t1079: f64, t11121: f64, t11201: f64, t16284: f64, t1651: f64, t1695: f64, t1696: f64, t20211: f64, t23583: f64, t23598: f64, t23603: f64, t23607: f64, t23617: f64, t24047: f64, t24177: f64, t24178: f64, t3058: f64, t3269: f64, t386: f64, t4747: f64, t4778: f64, t4935: f64, t6251: f64, t6350: f64, t80833: f64, t80992: f64, t995: f64, t996: f64, t6305: f64, t373: f64, t6299: f64, t1042: f64, t1063: f64, t1066: f64, t11875: f64, t15707: f64, t15716: f64, t1592: f64, t23844: f64, t23848: f64, t23852: f64, t247: f64, t3117: f64, t3127: f64, t3150: f64, t3155: f64, t3162: f64, t42868: f64, t42873: f64, t42984: f64, t42985: f64, t4834: f64, t6263: f64, t6271: f64, t65292: f64, t65717: f64, t78512: f64, t78550: f64, t78607: f64, t79301: f64, t88083: f64, t5819: f64, t5825: f64, t15696: f64, t15935: f64, t1671: f64, t19738: f64, t19878: f64, t22671: f64, t23863: f64, t23899: f64, t23931: f64, t23939: f64, t3161: f64, t43082: f64, t4806: f64, t4837: f64, t4872: f64, t55141: f64, t65357: f64, t78561: f64, t78564: f64, t78576: f64, t78583: f64, t79038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88607, t88628, t88646, t88660) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685(t3011, t3014, t88351, t981, t6392, t6244, t6258, t42013, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88673 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88675, t88682) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1687(t341, t88660, t88673, t1076, t1079, t11121, t11201, t16284, t1651, t1695, t1696, t20211, t23583, t23598, t23603, t23607, t23617, t24047, t24177, t24178, t3058, t3269, t386, t4747, t4778, t4935, t6244, t6251, t6258, t6350, t80833, t80992, t88628, t88646, t995, t996);
        let (t88694, t88695, t88714, t88715, t88727) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688(t6305, t373, t6299, t1042, t1063, t1066, t11875, t15707, t15716, t1592, t23844, t23848, t23852, t247, t3117, t3127, t3150, t3155, t3162, t42868, t42873, t42984, t42985, t4834, t6263, t6271, t65292, t65717, t78512, t78550, t78607, t79301, t88083);
        let (t88732, t88750, t88763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689(t5819, t5825, t6244, t1042, t1063, t15696, t15935, t1651, t1671, t19738, t19878, t22671, t23863, t23899, t23931, t23939, t3127, t3161, t3162, t43082, t4806, t4837, t4872, t55141, t65357, t78561, t78564, t78576, t78583, t79038, t88715);
    (t88607, t88646, t88675, t88682, t88694, t88695, t88714, t88727, t88732, t88750, t88763)
}
