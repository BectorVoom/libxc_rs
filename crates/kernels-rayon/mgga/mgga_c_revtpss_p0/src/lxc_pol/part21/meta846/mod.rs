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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta846(t43828: f64, t43830: f64, t43832: f64, t43911: f64, t56174: f64, t56176: f64, t56181: f64, t58055: f64, t58057: f64, t58060: f64, t58063: f64, t58107: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t58145: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t58138: f64, t58141: f64, t58143: f64, t58147: f64, t43858: f64, t43928: f64, t58151: f64, t58153: f64, t58156: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58168: f64, t58171: f64, t58174: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t58186: f64, t58189: f64, t58192: f64, t58195: f64, t58198: f64, t58225: f64, t56248: f64, t56252: f64, t56256: f64, t58202: f64, t58207: f64, t58209: f64, t58211: f64, t58214: f64, t58217: f64, t58220: f64, t58223: f64, t58359: f64, t58372: f64, t1130: f64, t16807: f64, t1151: f64, t16835: f64, t3428: f64, t1180: f64, t1188: f64, t12494: f64, t12497: f64, t17097: f64, t17151: f64, t3454: f64, t3480: f64, t3491: f64, t58317: f64, t58322: f64, t58325: f64, t58327: f64, t58330: f64, t58333: f64, t58336: f64, t58341: f64, t58344: f64, t58345: f64, t3432: f64, t5060: f64, t3436: f64, t12358: f64, t5063: f64, t12226: f64, t1719: f64, t12231: f64, t1733: f64, t45041: f64, t12238: f64, t5105: f64, t16943: f64, t3379: f64, t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t44039: f64, t44040: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t58029: f64, t58032: f64, t58035: f64, t58038: f64, t58041: f64, t58044: f64, t58046: f64, t58048: f64, t58051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t58386 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3165(t43828, t43830, t43832, t43911, t56174, t56176, t56181, t58055, t58057, t58060, t58063, t58107);
        let t58399 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3166(t56183, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56212, t56214, t56216);
        let (t58413, t58426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3167(t56228, t58145, t56221, t56226, t56230, t56234, t56236, t58138, t58141, t58143, t58147, t43858, t43928, t58151, t58153, t58156, t58158, t58160, t58162, t58165, t58168, t58171, t58174);
        let t58440 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3168(t43865, t43883, t43888, t43890, t43892, t43894, t43896, t58186, t58189, t58192, t58195, t58198);
        let t58453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3169(t58225, t56248, t56252, t56256, t58202, t58207, t58209, t58211, t58214, t58217, t58220, t58223);
        let (t58456, t58462, t58464) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3170(t58359, t58372, t58386, t58399, t58413, t58426, t58440, t58453, t1130, t16807, t1151, t16835, t3428);
        let t58465 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3171(t1180, t1188, t12494, t12497, t17097, t17151, t3454, t3480, t3491, t58317, t58322, t58325, t58327, t58330, t58333, t58336, t58341, t58344, t58345, t58456, t58462, t58464);
        let (t58468, t58472, t58475, t58477, t58479) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3172(t3432, t5060, t3436, t12358, t5063, t12226, t1719, t12231, t1733, t45041, t12238, t5105);
        let (t58481, t58491) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3173(t16943, t3379, t43762, t43771, t43773, t43781, t43783, t43785, t43787, t44039, t44040, t56151, t56155);
        let t58504 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3174(t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051);
    (t58456, t58462, t58464, t58465, t58468, t58472, t58475, t58477, t58479, t58481, t58491, t58504)
}
