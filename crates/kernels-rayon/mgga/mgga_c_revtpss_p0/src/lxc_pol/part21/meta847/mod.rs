//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta847 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3175;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3176;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3177;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3178;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3179;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3180;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3181;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3182;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3183;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3184;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta847(t43828: f64, t43830: f64, t43832: f64, t43911: f64, t56174: f64, t56176: f64, t56181: f64, t58055: f64, t58057: f64, t58060: f64, t58063: f64, t58107: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t58145: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t58138: f64, t58141: f64, t58143: f64, t58147: f64, t43858: f64, t43928: f64, t58151: f64, t58153: f64, t58156: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58168: f64, t58171: f64, t58174: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t58186: f64, t58189: f64, t58192: f64, t58195: f64, t58198: f64, t56248: f64, t56252: f64, t56256: f64, t58202: f64, t58207: f64, t58209: f64, t58211: f64, t58214: f64, t58217: f64, t58220: f64, t58223: f64, t58225: f64, t1131: f64, t1150: f64, t58491: f64, t58504: f64, t12470: f64, t1744: f64, t12364: f64, t16840: f64, t45232: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t12555: f64, t5180: f64, t1168: f64, t12465: f64, t12472: f64, t12547: f64, t12553: f64, t16988: f64, t3471: f64, t3497: f64, t3515: f64, t3521: f64, t435: f64, t5120: f64, t5184: f64, t56260: f64, t58468: f64, t58472: f64, t58475: f64, t58477: f64, t58479: f64, t58481: f64, t300: f64, t57943: f64, t57967: f64, t58004: f64, t58250: f64, t58275: f64, t58315: f64, t58465: f64, t16677: f64, t3531: f64, t16685: f64, t12571: f64, t5207: f64, t12486: f64, t1187: f64, t16812: f64, t16997: f64, t1196: f64, t16672: f64, t3498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t58518 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3175(t43828, t43830, t43832, t43911, t56174, t56176, t56181, t58055, t58057, t58060, t58063, t58107);
        let t58531 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3176(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
        let (t58545, t58558) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3177(t56228, t58145, t56221, t56226, t56230, t56234, t56236, t58138, t58141, t58143, t58147, t43858, t43928, t58151, t58153, t58156, t58158, t58160, t58162, t58165, t58168, t58171, t58174);
        let t58572 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3178(t43865, t43883, t43888, t43890, t43892, t43894, t43896, t58186, t58189, t58192, t58195, t58198);
        let t58585 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3179(t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223, t58225);
        let (t58591, t58592) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3180(t1131, t1150, t58491, t58504, t58518, t58531, t58545, t58558, t58572, t58585, t12470, t1744);
        let (t58598, t58618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3181(t12364, t16840, t56176, t56183, t43830, t43832, t45232, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t58639 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3182(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let t58654 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3183(t12555, t5180, t1168, t12465, t12472, t12547, t12553, t16988, t3471, t3497, t3515, t3521, t435, t5120, t5184, t56260, t58468, t58472, t58475, t58477, t58479, t58481, t58591, t58592, t58598, t58618, t58639);
        let (t58658, t58660, t58662) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3184(t300, t57943, t57967, t58004, t58250, t58275, t58315, t58465, t58654, t16677, t3531, t16685);
        let (t58664, t58666, t58669, t58671, t58675, t58678) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3185(t12571, t5207, t12486, t300, t1187, t3515, t5184, t16812, t3531, t12553, t16997, t1196, t16672, t3498);
    (t58591, t58598, t58658, t58660, t58662, t58664, t58666, t58669, t58671, t58675, t58678)
}
