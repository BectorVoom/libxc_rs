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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta847<F: Float>(t43828: F, t43830: F, t43832: F, t43911: F, t56174: F, t56176: F, t56181: F, t58055: F, t58057: F, t58060: F, t58063: F, t58107: F, t56183: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t58145: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t58138: F, t58141: F, t58143: F, t58147: F, t43858: F, t43928: F, t58151: F, t58153: F, t58156: F, t58158: F, t58160: F, t58162: F, t58165: F, t58168: F, t58171: F, t58174: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t58186: F, t58189: F, t58192: F, t58195: F, t58198: F, t56248: F, t56252: F, t56256: F, t58202: F, t58207: F, t58209: F, t58211: F, t58214: F, t58217: F, t58220: F, t58223: F, t58225: F, t1131: F, t1150: F, t58491: F, t58504: F, t12470: F, t1744: F, t12364: F, t16840: F, t45232: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t12555: F, t5180: F, t1168: F, t12465: F, t12472: F, t12547: F, t12553: F, t16988: F, t3471: F, t3497: F, t3515: F, t3521: F, t435: F, t5120: F, t5184: F, t56260: F, t58468: F, t58472: F, t58475: F, t58477: F, t58479: F, t58481: F, t300: F, t57943: F, t57967: F, t58004: F, t58250: F, t58275: F, t58315: F, t58465: F, t16677: F, t3531: F, t16685: F, t12571: F, t5207: F, t12486: F, t1187: F, t16812: F, t16997: F, t1196: F, t16672: F, t3498: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t58518 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3175::<F>(t43828, t43830, t43832, t43911, t56174, t56176, t56181, t58055, t58057, t58060, t58063, t58107);
        let t58531 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3176::<F>(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
        let (t58545, t58558) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3177::<F>(t56228, t58145, t56221, t56226, t56230, t56234, t56236, t58138, t58141, t58143, t58147, t43858, t43928, t58151, t58153, t58156, t58158, t58160, t58162, t58165, t58168, t58171, t58174);
        let t58572 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3178::<F>(t43865, t43883, t43888, t43890, t43892, t43894, t43896, t58186, t58189, t58192, t58195, t58198);
        let t58585 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3179::<F>(t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223, t58225);
        let (t58591, t58592) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3180::<F>(t1131, t1150, t58491, t58504, t58518, t58531, t58545, t58558, t58572, t58585, t12470, t1744);
        let (t58598, t58618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3181::<F>(t12364, t16840, t56176, t56183, t43830, t43832, t45232, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t58639 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3182::<F>(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let t58654 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3183::<F>(t12555, t5180, t1168, t12465, t12472, t12547, t12553, t16988, t3471, t3497, t3515, t3521, t435, t5120, t5184, t56260, t58468, t58472, t58475, t58477, t58479, t58481, t58591, t58592, t58598, t58618, t58639);
        let (t58658, t58660, t58662) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3184::<F>(t300, t57943, t57967, t58004, t58250, t58275, t58315, t58465, t58654, t16677, t3531, t16685);
        let (t58664, t58666, t58669, t58671, t58675, t58678) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3185::<F>(t12571, t5207, t12486, t300, t1187, t3515, t5184, t16812, t3531, t12553, t16997, t1196, t16672, t3498);
    (t58591, t58598, t58658, t58660, t58662, t58664, t58666, t58669, t58671, t58675, t58678)
}
