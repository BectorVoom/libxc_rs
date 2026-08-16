//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1053 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3719;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1053<F: Float>(t3671: F, t371: F, t6609: F, t676: F, t5245: F, t1235: F, t127: F, t21083: F, t12967: F, t20846: F, t17708: F, t59550: F, t12916: F, t21299: F, t3718: F, t17484: F, t17748: F, t17754: F, t17756: F, t372: F, t3720: F, t482: F, t57241: F, t57250: F, t57252: F, t57256: F, t57258: F, t57382: F, t59011: F, t59196: F, t70235: F, t17237: F, t17381: F, t5381: F, t57270: F, t57273: F, t57290: F, t57292: F, t57295: F, t57297: F, t57299: F, t57314: F, t57316: F, t57318: F, t57321: F, t480: F, t69637: F, t20842: F, t3667: F, t17303: F, t5323: F, t12784: F, t17401: F, t17515: F, t17534: F, t17654: F, t17662: F, t17729: F, t17744: F, t20766: F, t21161: F, t3626: F, t3674: F, t5051: F, t56981: F, t57331: F, t57333: F, t57336: F, t57660: F, t57663: F, t57710: F, t12866: F, t5406: F, t58895: F, t17789: F, t21306: F, t17617: F, t15687: F, t17394: F, t3782: F, t1122: F, t5284: F, t12809: F, t13396: F, t17353: F, t17355: F, t17605: F, t17640: F, t17753: F, t17784: F, t20938: F, t20956: F, t3603: F, t3604: F, t44578: F, t44585: F, t45371: F, t471: F, t5046: F, t56760: F, t56888: F, t57005: F, t57386: F, t6688: F) -> (F, F, F, F, F, F, F) {
        let (t70511, t70513, t70521, t70523, t70530) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3719::<F>(t3671, t371, t6609, t676, t5245, t1235, t127, t21083, t12967, t20846, t17708, t59550);
        let t70546 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720::<F>(t12916, t21299, t3718, t17484, t17748, t17754, t17756, t3671, t371, t372, t3720, t482, t57241, t57250, t57252, t57256, t57258, t57382, t59011, t59196, t70235, t70511, t70513, t70521, t70523, t70530);
        let t70565 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721::<F>(t17237, t17381, t5381, t57270, t57273, t57290, t57292, t57295, t57297, t57299, t57314, t57316, t57318, t57321, t57382);
        let t70593 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3722::<F>(t480, t69637, t20842, t3667, t17303, t5323, t12784, t17401, t17484, t17515, t17534, t17654, t17662, t17729, t17744, t20766, t21161, t3626, t3674, t5051, t56981, t57331, t57333, t57336, t57660, t57663, t57710);
        let (t70612, t70616, t70623, t70629, t70630, t70633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723::<F>(t12866, t5406, t58895, t17789, t21306, t17401, t17617, t15687, t17394, t3782, t1122, t5284);
        let t70638 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724::<F>(t12809, t13396, t17353, t17355, t17605, t17640, t17654, t17753, t17784, t20938, t20956, t3603, t3604, t3626, t3720, t44578, t44585, t45371, t471, t5046, t56760, t56888, t57005, t57386, t6688, t70612, t70616, t70623, t70630, t70633);
    (t70513, t70546, t70565, t70593, t70629, t70633, t70638)
}
