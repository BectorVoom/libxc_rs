//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta801(t52126: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41441: f64, t52112: f64, t52128: f64, t52130: f64, t52664: f64, t52677: f64, t52690: f64, t52702: f64, t52716: f64, t52729: f64, t52743: f64, t915: f64, t935: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t41549: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t291: f64, t11531: f64, t15421: f64, t2942: f64, t4644: f64, t11408: f64, t1614: f64, t11411: f64, t11502: f64, t11510: f64, t15343: f64, t1634: f64, t2945: f64, t3007: f64, t3015: f64, t41746: f64, t4685: f64, t52522: f64, t52536: f64, t52549: f64, t52562: f64, t52574: f64, t52588: f64, t52601: f64, t52615: f64, t52628: f64, t52637: f64, t52642: f64, t52647: f64, t52650: f64, t52652: f64, t946: f64, t954: f64, t974: f64, t2967: f64, t11449: f64, t15373: f64, t945: f64, t11409: f64, t1621: f64, t2968: f64, t11445: f64, t11453: f64, t11456: f64, t11466: f64, t11467: f64, t11513: f64, t11517: f64, t11525: f64, t15104: f64, t15235: f64, t15339: f64, t15350: f64, t15400: f64, t15406: f64, t1622: f64, t2963: f64, t2970: f64, t2971: f64, t2982: f64, t41794: f64, t4647: f64, t4708: f64, t953: f64, t955: f64, t300: f64, t52282: f64, t52324: f64, t52377: f64, t52433: f64, t52477: f64, t52520: f64, t4724: f64, t981: f64, t11606: f64, t4719: f64, t1642: f64, t41491: f64, t11591: f64, t4729: f64, t52229: f64, t52231: f64, t52235: f64, t52237: f64, t52242: f64, t52245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t52756 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907(t52126, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t41441, t52112, t52128, t52130);
        let (t52762, t52782) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2908(t52664, t52677, t52690, t52702, t52716, t52729, t52743, t52756, t915, t935, t51973, t41361, t41363, t41369, t41549, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52803 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52806, t52808, t52817) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910(t291, t52782, t52803, t11531, t15421, t2942, t4644, t11408, t1614, t11411, t11502, t11510, t15343, t1634, t2945, t3007, t3015, t41746, t4685, t52522, t52536, t52549, t52562, t52574, t52588, t52601, t52615, t52628, t52637, t52642, t52647, t52650, t52652, t52762, t946, t954, t974);
        let t52856 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2911(t2967, t4644, t11449, t1614, t15373, t945, t11409, t1621, t2968, t11445, t11453, t11456, t11466, t11467, t11513, t11517, t11525, t15104, t15235, t15339, t15350, t15400, t15406, t1622, t1634, t2963, t2970, t2971, t2982, t41794, t4647, t4708, t953, t955);
        let (t52860, t52863) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2912(t300, t52282, t52324, t52377, t52433, t52477, t52520, t52817, t52856, t11502, t4724, t981);
        let (t52865, t52867, t52869, t52870) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913(t11606, t4719, t1642, t41491, t11591, t4729, t52229, t52231, t52235, t52237, t52242, t52245, t52860, t52863);
    (t52762, t52806, t52808, t52860, t52863, t52865, t52867, t52869, t52870)
}
