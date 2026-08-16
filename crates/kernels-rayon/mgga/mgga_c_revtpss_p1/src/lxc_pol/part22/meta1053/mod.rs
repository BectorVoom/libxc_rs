//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1053 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3719;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1053(t3671: f64, t371: f64, t6609: f64, t676: f64, t5245: f64, t1235: f64, t127: f64, t21083: f64, t12967: f64, t20846: f64, t17708: f64, t59550: f64, t12916: f64, t21299: f64, t3718: f64, t17484: f64, t17748: f64, t17754: f64, t17756: f64, t372: f64, t3720: f64, t482: f64, t57241: f64, t57250: f64, t57252: f64, t57256: f64, t57258: f64, t57382: f64, t59011: f64, t59196: f64, t70235: f64, t17237: f64, t17381: f64, t5381: f64, t57270: f64, t57273: f64, t57290: f64, t57292: f64, t57295: f64, t57297: f64, t57299: f64, t57314: f64, t57316: f64, t57318: f64, t57321: f64, t480: f64, t69637: f64, t20842: f64, t3667: f64, t17303: f64, t5323: f64, t12784: f64, t17401: f64, t17515: f64, t17534: f64, t17654: f64, t17662: f64, t17729: f64, t17744: f64, t20766: f64, t21161: f64, t3626: f64, t3674: f64, t5051: f64, t56981: f64, t57331: f64, t57333: f64, t57336: f64, t57660: f64, t57663: f64, t57710: f64, t12866: f64, t5406: f64, t58895: f64, t17789: f64, t21306: f64, t17617: f64, t15687: f64, t17394: f64, t3782: f64, t1122: f64, t5284: f64, t12809: f64, t13396: f64, t17353: f64, t17355: f64, t17605: f64, t17640: f64, t17753: f64, t17784: f64, t20938: f64, t20956: f64, t3603: f64, t3604: f64, t44578: f64, t44585: f64, t45371: f64, t471: f64, t5046: f64, t56760: f64, t56888: f64, t57005: f64, t57386: f64, t6688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t70511, t70513, t70521, t70523, t70530) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3719(t3671, t371, t6609, t676, t5245, t1235, t127, t21083, t12967, t20846, t17708, t59550);
        let t70546 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720(t12916, t21299, t3718, t17484, t17748, t17754, t17756, t3671, t371, t372, t3720, t482, t57241, t57250, t57252, t57256, t57258, t57382, t59011, t59196, t70235, t70511, t70513, t70521, t70523, t70530);
        let t70565 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721(t17237, t17381, t5381, t57270, t57273, t57290, t57292, t57295, t57297, t57299, t57314, t57316, t57318, t57321, t57382);
        let t70593 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722(t480, t69637, t20842, t3667, t17303, t5323, t12784, t17401, t17484, t17515, t17534, t17654, t17662, t17729, t17744, t20766, t21161, t3626, t3674, t5051, t56981, t57331, t57333, t57336, t57660, t57663, t57710);
        let (t70612, t70616, t70623, t70629, t70630, t70633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723(t12866, t5406, t58895, t17789, t21306, t17401, t17617, t15687, t17394, t3782, t1122, t5284);
        let t70638 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724(t12809, t13396, t17353, t17355, t17605, t17640, t17654, t17753, t17784, t20938, t20956, t3603, t3604, t3626, t3720, t44578, t44585, t45371, t471, t5046, t56760, t56888, t57005, t57386, t6688, t70612, t70616, t70623, t70630, t70633);
    (t70513, t70546, t70565, t70593, t70629, t70633, t70638)
}
