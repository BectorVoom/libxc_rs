//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta561 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1687;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta561<F: Float>(t3011: F, t3014: F, t88351: F, t981: F, t6392: F, t6244: F, t6258: F, t42013: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F, t341: F, t1076: F, t1079: F, t11121: F, t11201: F, t16284: F, t1651: F, t1695: F, t1696: F, t20211: F, t23583: F, t23598: F, t23603: F, t23607: F, t23617: F, t24047: F, t24177: F, t24178: F, t3058: F, t3269: F, t386: F, t4747: F, t4778: F, t4935: F, t6251: F, t6350: F, t80833: F, t80992: F, t995: F, t996: F, t6305: F, t373: F, t6299: F, t1042: F, t1063: F, t1066: F, t11875: F, t15707: F, t15716: F, t1592: F, t23844: F, t23848: F, t23852: F, t247: F, t3117: F, t3127: F, t3150: F, t3155: F, t3162: F, t42868: F, t42873: F, t42984: F, t42985: F, t4834: F, t6263: F, t6271: F, t65292: F, t65717: F, t78512: F, t78550: F, t78607: F, t79301: F, t88083: F, t5819: F, t5825: F, t15696: F, t15935: F, t1671: F, t19738: F, t19878: F, t22671: F, t23863: F, t23899: F, t23931: F, t23939: F, t3161: F, t43082: F, t4806: F, t4837: F, t4872: F, t55141: F, t65357: F, t78561: F, t78564: F, t78576: F, t78583: F, t79038: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88607, t88628, t88646, t88660) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1685::<F>(t3011, t3014, t88351, t981, t6392, t6244, t6258, t42013, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88673 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686::<F>(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88675, t88682) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1687::<F>(t341, t88660, t88673, t1076, t1079, t11121, t11201, t16284, t1651, t1695, t1696, t20211, t23583, t23598, t23603, t23607, t23617, t24047, t24177, t24178, t3058, t3269, t386, t4747, t4778, t4935, t6244, t6251, t6258, t6350, t80833, t80992, t88628, t88646, t995, t996);
        let (t88694, t88695, t88714, t88715, t88727) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1688::<F>(t6305, t373, t6299, t1042, t1063, t1066, t11875, t15707, t15716, t1592, t23844, t23848, t23852, t247, t3117, t3127, t3150, t3155, t3162, t42868, t42873, t42984, t42985, t4834, t6263, t6271, t65292, t65717, t78512, t78550, t78607, t79301, t88083);
        let (t88732, t88750, t88763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689::<F>(t5819, t5825, t6244, t1042, t1063, t15696, t15935, t1651, t1671, t19738, t19878, t22671, t23863, t23899, t23931, t23939, t3127, t3161, t3162, t43082, t4806, t4837, t4872, t55141, t65357, t78561, t78564, t78576, t78583, t79038, t88715);
    (t88607, t88646, t88675, t88682, t88694, t88695, t88714, t88727, t88732, t88750, t88763)
}
