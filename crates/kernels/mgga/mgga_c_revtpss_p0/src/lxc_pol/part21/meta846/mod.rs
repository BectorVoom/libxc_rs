//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta846 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3165;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3166;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3167;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3168;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3169;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3170;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3171;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3172;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3173;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta846<F: Float>(t43828: F, t43830: F, t43832: F, t43911: F, t56174: F, t56176: F, t56181: F, t58055: F, t58057: F, t58060: F, t58063: F, t58107: F, t56183: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t58145: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t58138: F, t58141: F, t58143: F, t58147: F, t43858: F, t43928: F, t58151: F, t58153: F, t58156: F, t58158: F, t58160: F, t58162: F, t58165: F, t58168: F, t58171: F, t58174: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t58186: F, t58189: F, t58192: F, t58195: F, t58198: F, t58225: F, t56248: F, t56252: F, t56256: F, t58202: F, t58207: F, t58209: F, t58211: F, t58214: F, t58217: F, t58220: F, t58223: F, t58359: F, t58372: F, t1130: F, t16807: F, t1151: F, t16835: F, t3428: F, t1180: F, t1188: F, t12494: F, t12497: F, t17097: F, t17151: F, t3454: F, t3480: F, t3491: F, t58317: F, t58322: F, t58325: F, t58327: F, t58330: F, t58333: F, t58336: F, t58341: F, t58344: F, t58345: F, t3432: F, t5060: F, t3436: F, t12358: F, t5063: F, t12226: F, t1719: F, t12231: F, t1733: F, t45041: F, t12238: F, t5105: F, t16943: F, t3379: F, t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t44039: F, t44040: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t58029: F, t58032: F, t58035: F, t58038: F, t58041: F, t58044: F, t58046: F, t58048: F, t58051: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t58386 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3165::<F>(t43828, t43830, t43832, t43911, t56174, t56176, t56181, t58055, t58057, t58060, t58063, t58107);
        let t58399 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3166::<F>(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
        let (t58413, t58426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3167::<F>(t56228, t58145, t56221, t56226, t56230, t56234, t56236, t58138, t58141, t58143, t58147, t43858, t43928, t58151, t58153, t58156, t58158, t58160, t58162, t58165, t58168, t58171, t58174);
        let t58440 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3168::<F>(t43865, t43883, t43888, t43890, t43892, t43894, t43896, t58186, t58189, t58192, t58195, t58198);
        let t58453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3169::<F>(t58225, t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223);
        let (t58456, t58462, t58464) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3170::<F>(t58359, t58372, t58386, t58399, t58413, t58426, t58440, t58453, t1130, t16807, t1151, t16835, t3428);
        let t58465 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3171::<F>(t1180, t1188, t12494, t12497, t17097, t17151, t3454, t3480, t3491, t58317, t58322, t58325, t58327, t58330, t58333, t58336, t58341, t58344, t58345, t58456, t58462, t58464);
        let (t58468, t58472, t58475, t58477, t58479) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3172::<F>(t3432, t5060, t3436, t12358, t5063, t12226, t1719, t12231, t1733, t45041, t12238, t5105);
        let (t58481, t58491) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3173::<F>(t16943, t3379, t43762, t43771, t43773, t43781, t43783, t43785, t43787, t44039, t44040, t56151, t56155);
        let t58504 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3174::<F>(t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051);
    (t58456, t58462, t58464, t58465, t58468, t58472, t58475, t58477, t58479, t58481, t58491, t58504)
}
