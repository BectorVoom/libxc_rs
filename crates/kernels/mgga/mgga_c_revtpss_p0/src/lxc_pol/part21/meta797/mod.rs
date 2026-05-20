//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta797 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2881;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2882;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2883;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2884;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2885;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2886;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2887;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2888;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta797<F: Float>(t3006: F, t972: F, t4711: F, t52238: F, t11557: F, t15572: F, t981: F, t11452: F, t4669: F, t11404: F, t11410: F, t11444: F, t11450: F, t11521: F, t11548: F, t11554: F, t15238: F, t15242: F, t15249: F, t15252: F, t15255: F, t15274: F, t15283: F, t15284: F, t15413: F, t1621: F, t2944: F, t2962: F, t2968: F, t41662: F, t41740: F, t41742: F, t41775: F, t41785: F, t41788: F, t41799: F, t4652: F, t4673: F, t4674: F, t4690: F, t11409: F, t11461: F, t11466: F, t11501: F, t15235: F, t15241: F, t15258: F, t15259: F, t15263: F, t15267: F, t15287: F, t15340: F, t1622: F, t1634: F, t2943: F, t2987: F, t2988: F, t41667: F, t41751: F, t41756: F, t41895: F, t4670: F, t4708: F, t4712: F, t953: F, t51973: F, t41361: F, t41363: F, t41369: F, t41520: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F, t52035: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F, t324: F, t11507: F, t1633: F, t41813: F, t52153: F, t52156: F, t52159: F, t52162: F, t52166: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t52182: F, t52185: F, t41908: F, t3012: F, t11467: F, t15290: F, t311: F, t52207: F, t52209: F, t52211: F, t52213: F, t52216: F, t52218: F, t52221: F, t52223: F, t52226: F, t52229: F, t2986: F, t4682: F, t11465: F, t1626: F, t15234: F, t3014: F, t11509: F, t4707: F, t11399: F, t11468: F, t15266: F, t15277: F, t15280: F, t2938: F, t2989: F, t41238: F, t41658: F, t41759: F, t41779: F, t52231: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t52239, t52242, t52245, t52282) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2881::<F>(t3006, t972, t4711, t52238, t11557, t15572, t981, t11452, t4669, t11404, t11410, t11444, t11450, t11521, t11548, t11554, t15238, t15242, t15249, t15252, t15255, t15274, t15283, t15284, t15413, t1621, t2944, t2962, t2968, t41662, t41740, t41742, t41775, t41785, t41788, t41799, t4652, t4673, t4674, t4690);
        let t52324 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2882::<F>(t11450, t1621, t11404, t11409, t11410, t11444, t11461, t11466, t11501, t15235, t15241, t15258, t15259, t15263, t15267, t15283, t15287, t15340, t1622, t1634, t2943, t2944, t2962, t2987, t2988, t3006, t41667, t41751, t41756, t41895, t4670, t4708, t4712, t953, t972);
        let t52345 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2883::<F>(t51973, t41361, t41363, t41369, t41520, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52366 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2884::<F>(t52035, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52368, t52377) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2885::<F>(t324, t52345, t52366, t11507, t1633, t11409, t11410, t1622, t41813, t52153, t52156, t52159, t52162, t52166, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t972);
        let t52405 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2886::<F>(t51973, t41361, t41363, t41369, t41908, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52426 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2887::<F>(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let t52433 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2888::<F>(t1633, t3012, t11410, t11450, t11461, t11467, t11507, t11521, t15290, t311, t4673, t4711, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223, t52226, t52229, t52405, t52426);
        let t52477 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2889::<F>(t2986, t4682, t11465, t1626, t15234, t3014, t11509, t4707, t11399, t11467, t11468, t11501, t11507, t11548, t15258, t15266, t15277, t15280, t15340, t1633, t2938, t2944, t2968, t2988, t2989, t3006, t3012, t41238, t41658, t41759, t41779, t4670, t4708, t4711, t52231, t972);
    (t52239, t52242, t52245, t52282, t52324, t52368, t52377, t52433, t52477)
}
