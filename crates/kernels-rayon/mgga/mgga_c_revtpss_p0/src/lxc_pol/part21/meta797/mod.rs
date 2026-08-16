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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta797(t3006: f64, t972: f64, t4711: f64, t52238: f64, t11557: f64, t15572: f64, t981: f64, t11452: f64, t4669: f64, t11404: f64, t11410: f64, t11444: f64, t11450: f64, t11521: f64, t11548: f64, t11554: f64, t15238: f64, t15242: f64, t15249: f64, t15252: f64, t15255: f64, t15274: f64, t15283: f64, t15284: f64, t15413: f64, t1621: f64, t2944: f64, t2962: f64, t2968: f64, t41662: f64, t41740: f64, t41742: f64, t41775: f64, t41785: f64, t41788: f64, t41799: f64, t4652: f64, t4673: f64, t4674: f64, t4690: f64, t11409: f64, t11461: f64, t11466: f64, t11501: f64, t15235: f64, t15241: f64, t15258: f64, t15259: f64, t15263: f64, t15267: f64, t15287: f64, t15340: f64, t1622: f64, t1634: f64, t2943: f64, t2987: f64, t2988: f64, t41667: f64, t41751: f64, t41756: f64, t41895: f64, t4670: f64, t4708: f64, t4712: f64, t953: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t41520: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64, t324: f64, t11507: f64, t1633: f64, t41813: f64, t52153: f64, t52156: f64, t52159: f64, t52162: f64, t52166: f64, t52170: f64, t52174: f64, t52176: f64, t52178: f64, t52180: f64, t52182: f64, t52185: f64, t41908: f64, t3012: f64, t11467: f64, t15290: f64, t311: f64, t52207: f64, t52209: f64, t52211: f64, t52213: f64, t52216: f64, t52218: f64, t52221: f64, t52223: f64, t52226: f64, t52229: f64, t2986: f64, t4682: f64, t11465: f64, t1626: f64, t15234: f64, t3014: f64, t11509: f64, t4707: f64, t11399: f64, t11468: f64, t15266: f64, t15277: f64, t15280: f64, t2938: f64, t2989: f64, t41238: f64, t41658: f64, t41759: f64, t41779: f64, t52231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52239, t52242, t52245, t52282) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2881(t3006, t972, t4711, t52238, t11557, t15572, t981, t11452, t4669, t11404, t11410, t11444, t11450, t11521, t11548, t11554, t15238, t15242, t15249, t15252, t15255, t15274, t15283, t15284, t15413, t1621, t2944, t2962, t2968, t41662, t41740, t41742, t41775, t41785, t41788, t41799, t4652, t4673, t4674, t4690);
        let t52324 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2882(t11450, t1621, t11404, t11409, t11410, t11444, t11461, t11466, t11501, t15235, t15241, t15258, t15259, t15263, t15267, t15283, t15287, t15340, t1622, t1634, t2943, t2944, t2962, t2987, t2988, t3006, t41667, t41751, t41756, t41895, t4670, t4708, t4712, t953, t972);
        let t52345 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2883(t51973, t41361, t41363, t41369, t41520, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52366 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2884(t52035, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52368, t52377) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2885(t324, t52345, t52366, t11507, t1633, t11409, t11410, t1622, t41813, t52153, t52156, t52159, t52162, t52166, t52170, t52174, t52176, t52178, t52180, t52182, t52185, t972);
        let t52405 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2886(t51973, t41361, t41363, t41369, t41908, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52426 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2887(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let t52433 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2888(t1633, t3012, t11410, t11450, t11461, t11467, t11507, t11521, t15290, t311, t4673, t4711, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223, t52226, t52229, t52405, t52426);
        let t52477 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2889(t2986, t4682, t11465, t1626, t15234, t3014, t11509, t4707, t11399, t11467, t11468, t11501, t11507, t11548, t15258, t15266, t15277, t15280, t15340, t1633, t2938, t2944, t2968, t2988, t2989, t3006, t3012, t41238, t41658, t41759, t41779, t4670, t4708, t4711, t52231, t972);
    (t52239, t52242, t52245, t52282, t52324, t52368, t52377, t52433, t52477)
}
