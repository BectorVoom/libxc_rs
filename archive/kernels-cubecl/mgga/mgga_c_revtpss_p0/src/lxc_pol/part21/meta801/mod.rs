//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta801<F: Float>(t52126: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41441: F, t52112: F, t52128: F, t52130: F, t52664: F, t52677: F, t52690: F, t52702: F, t52716: F, t52729: F, t52743: F, t915: F, t935: F, t51973: F, t41361: F, t41363: F, t41369: F, t41549: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t291: F, t11531: F, t15421: F, t2942: F, t4644: F, t11408: F, t1614: F, t11411: F, t11502: F, t11510: F, t15343: F, t1634: F, t2945: F, t3007: F, t3015: F, t41746: F, t4685: F, t52522: F, t52536: F, t52549: F, t52562: F, t52574: F, t52588: F, t52601: F, t52615: F, t52628: F, t52637: F, t52642: F, t52647: F, t52650: F, t52652: F, t946: F, t954: F, t974: F, t2967: F, t11449: F, t15373: F, t945: F, t11409: F, t1621: F, t2968: F, t11445: F, t11453: F, t11456: F, t11466: F, t11467: F, t11513: F, t11517: F, t11525: F, t15104: F, t15235: F, t15339: F, t15350: F, t15400: F, t15406: F, t1622: F, t2963: F, t2970: F, t2971: F, t2982: F, t41794: F, t4647: F, t4708: F, t953: F, t955: F, t300: F, t52282: F, t52324: F, t52377: F, t52433: F, t52477: F, t52520: F, t4724: F, t981: F, t11606: F, t4719: F, t1642: F, t41491: F, t11591: F, t4729: F, t52229: F, t52231: F, t52235: F, t52237: F, t52242: F, t52245: F) -> (F, F, F, F, F, F, F, F, F) {
        let t52756 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907::<F>(t52126, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t41441, t52112, t52128, t52130);
        let (t52762, t52782) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908::<F>(t52664, t52677, t52690, t52702, t52716, t52729, t52743, t52756, t915, t935, t51973, t41361, t41363, t41369, t41549, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52803 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909::<F>(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52806, t52808, t52817) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910::<F>(t291, t52782, t52803, t11531, t15421, t2942, t4644, t11408, t1614, t11411, t11502, t11510, t15343, t1634, t2945, t3007, t3015, t41746, t4685, t52522, t52536, t52549, t52562, t52574, t52588, t52601, t52615, t52628, t52637, t52642, t52647, t52650, t52652, t52762, t946, t954, t974);
        let t52856 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911::<F>(t2967, t4644, t11449, t1614, t15373, t945, t11409, t1621, t2968, t11445, t11453, t11456, t11466, t11467, t11513, t11517, t11525, t15104, t15235, t15339, t15350, t15400, t15406, t1622, t1634, t2963, t2970, t2971, t2982, t41794, t4647, t4708, t953, t955);
        let (t52860, t52863) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912::<F>(t300, t52282, t52324, t52377, t52433, t52477, t52520, t52817, t52856, t11502, t4724, t981);
        let (t52865, t52867, t52869, t52870) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913::<F>(t11606, t4719, t1642, t41491, t11591, t4729, t52229, t52231, t52235, t52237, t52242, t52245, t52860, t52863);
    (t52762, t52806, t52808, t52860, t52863, t52865, t52867, t52869, t52870)
}
