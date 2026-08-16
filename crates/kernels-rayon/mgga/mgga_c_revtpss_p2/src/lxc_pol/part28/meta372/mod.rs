//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta372 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1404;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1405;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1406;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1407;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1408;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1409;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1410;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1411;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1412;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1413;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta372(t1466: f64, t2246: f64, t1497: f64, t2248: f64, t4241: f64, t644: f64, t2315: f64, t10355: f64, t1469: f64, t2251: f64, t2275: f64, t4186: f64, t30: f64, t33: f64, t606: f64, t2258: f64, t4201: f64, t580: f64, t9342: f64, zeta_threshold: f64, t48: f64, t10368: f64, t2282: f64, t4210: f64, t60: f64, t10379: f64, t1474: f64, t1480: f64, t2270: f64, t2283: f64, t2286: f64, t4202: f64, t4205: f64, t44: f64, t56: f64, t614: f64, t38: f64, t1486: f64, t2259: f64, t4217: f64, t607: f64, t1471: f64, t1487: f64, t1494: f64, t2252: f64, t2260: f64, t2263: f64, t2312: f64, t4196: f64, t4218: f64, t4238: f64, t608: f64, t641: f64, t85: f64, t10389: f64, t2299: f64, t10398: f64, t2306: f64, t4227: f64, t4232: f64, t633: f64, t637: f64, t77: f64, t70: f64, t4181: f64, t627: f64, t36: f64, t4187: f64, t1470: f64, t2291: f64, t2292: f64, t4182: f64, t4188: f64, t4191: f64, t628: f64, t71: f64, t10298: f64, t10301: f64, t10309: f64, t13267: f64, t13269: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t603: f64, t91: f64, t5: f64, t117: f64, t116: f64, t4245: f64, t1501: f64, t2327: f64, t648: f64, t670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1404(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13312) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1405(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
        let t13334 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1406(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
        let t13363 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1407(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
        let t13388 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1408(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1409(t13388, t77, t1469, t2258);
        let (t13393, t13396) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1410(t13392, t70, t4186, t606);
        let (t13405, t13419) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1411(t13396, t70, t4181, t627, t13312, t36, t4187, t1470, t2291, t13389, t13393, t1494, t2292, t4182, t4188, t4191, t4238, t628, t641, t71, t85);
        let (t13420, t13423) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1412(t13363, t13419, t10298, t10301, t10309, t13267, t13269, t13272, t13283, t13286, t13289, t1497, t2242, t2247, t2248, t2315, t4173, t4178, t4241, t603, t644, t91);
        let (t13424, t13425, t13426) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1413(t5, t13423, t117, t116, t4245);
        let (t13429, t13435) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1414(t1501, t2327, t648, t670);
    (t13272, t13312, t13388, t13392, t13396, t13405, t13420, t13424, t13425, t13426, t13429, t13435)
}
