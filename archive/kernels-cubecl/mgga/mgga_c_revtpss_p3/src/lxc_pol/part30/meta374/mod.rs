//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta374 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1402;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1403;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1404;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1405;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1406;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1407;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1408;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1409;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1410;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1411;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta374<F: Float>(t1466: F, t2246: F, t1497: F, t2248: F, t4241: F, t644: F, t2315: F, t10355: F, t1469: F, t2251: F, t2275: F, t4186: F, t30: F, t33: F, t606: F, t2258: F, t4201: F, t580: F, t9342: F, zeta_threshold: F, t48: F, t10368: F, t2282: F, t4210: F, t60: F, t10379: F, t1474: F, t1480: F, t2270: F, t2283: F, t2286: F, t4202: F, t4205: F, t44: F, t56: F, t614: F, t38: F, t1486: F, t2259: F, t4217: F, t607: F, t1471: F, t1487: F, t1494: F, t2252: F, t2260: F, t2263: F, t2312: F, t4196: F, t4218: F, t4238: F, t608: F, t641: F, t85: F, t10389: F, t2299: F, t10398: F, t2306: F, t4227: F, t4232: F, t633: F, t637: F, t77: F, t70: F, t4181: F, t627: F, t36: F, t4187: F, t1470: F, t2291: F, t2292: F, t4182: F, t4188: F, t4191: F, t628: F, t71: F, t10298: F, t10301: F, t10309: F, t13267: F, t13269: F, t2242: F, t2247: F, t4173: F, t4178: F, t603: F, t91: F, t5: F, t117: F, t116: F, t4245: F, t1501: F, t2327: F, t648: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1402::<F>(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13312) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1403::<F>(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
        let t13334 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1404::<F>(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
        let t13363 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1405::<F>(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
        let t13388 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1406::<F>(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1407::<F>(t13388, t77, t1469, t2258);
        let (t13393, t13396) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1408::<F>(t13392, t70, t4186, t606);
        let (t13405, t13419) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1409::<F>(t13396, t70, t4181, t627, t13312, t36, t4187, t1470, t2291, t13389, t13393, t1494, t2292, t4182, t4188, t4191, t4238, t628, t641, t71, t85);
        let (t13420, t13423) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1410::<F>(t13363, t13419, t10298, t10301, t10309, t13267, t13269, t13272, t13283, t13286, t13289, t1497, t2242, t2247, t2248, t2315, t4173, t4178, t4241, t603, t644, t91);
        let (t13424, t13425, t13426) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1411::<F>(t5, t13423, t117, t116, t4245);
        let (t13429, t13435) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1412::<F>(t1501, t2327, t648, t670);
    (t13272, t13312, t13388, t13392, t13396, t13405, t13420, t13424, t13425, t13426, t13429, t13435)
}
