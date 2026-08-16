//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3100;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3101;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3102;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3103;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta831(t3623: f64, t53739: f64, t13127: f64, t12865: f64, t3746: f64, t13396: f64, t5405: f64, t13392: f64, t17672: f64, t4181: f64, t17677: f64, t17682: f64, t1042: f64, t1261: f64, t1264: f64, t12712: f64, t17351: f64, t17353: f64, t17644: f64, t17654: f64, t17693: f64, t17694: f64, t17696: f64, t17799: f64, t17800: f64, t1797: f64, t247: f64, t3629: f64, t44248: f64, t44252: f64, t44264: f64, t44267: f64, t44270: f64, t44585: f64, t5302: f64, t54450: f64, t56232: f64, t1214: f64, t3611: f64, t12831: f64, t17395: f64, t12702: f64, t17350: f64, t1263: f64, t372: f64, t5284: f64, t1250: f64, t12809: f64, t12862: f64, t12866: f64, t13069: f64, t16696: f64, t16756: f64, t17396: f64, t17482: f64, t17512: f64, t17649: f64, t17657: f64, t17658: f64, t2251: f64, t2258: f64, t3367: f64, t3720: f64, t3723: f64, t44273: f64, t44276: f64, t44278: f64, t44280: f64, t44283: f64, t44286: f64, t44289: f64, t44510: f64, t44517: f64, t44952: f64, t5287: f64, t5297: f64, t13148: f64, t1121: f64, t3601: f64, t606: f64, t17728: f64, t460: f64, t489: f64, t17261: f64, t17373: f64, t12772: f64, t17639: f64, t3625: f64, t17645: f64, t1284: f64, t17288: f64, t3624: f64, t12917: f64, t17401: f64, t11231: f64, t12732: f64, t12855: f64, t12876: f64, t13046: f64, t16714: f64, t17454: f64, t17456: f64, t17609: f64, t17655: f64, t21017: f64, t3588: f64, t3591: f64, t3604: f64, t44225: f64, t44291: f64, t44293: f64, t44326: f64, t44484: f64, t45764: f64, t471: f64, t5330: f64, t5331: f64, t5332: f64, t5335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56878, t56879, t56888, t56891, t56895, t56899, t56903, t56907) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3100(t3623, t53739, t13127, t12865, t3746, t13396, t5405, t13392, t17672, t4181, t17677, t17682);
        let t56932 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3101(t1042, t1261, t1264, t12712, t17351, t17353, t17644, t17654, t17693, t17694, t17696, t17799, t17800, t1797, t247, t3629, t44248, t44252, t44264, t44267, t44270, t44585, t5302, t54450, t56232, t56879, t56888, t56891, t56895, t56899, t56903, t56907);
        let (t56981, t56985) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3102(t1214, t3611, t12831, t17395, t12702, t17350, t1263, t372, t5284, t1250, t12809, t12862, t12866, t13069, t16696, t16756, t17353, t17396, t17482, t17512, t17649, t17654, t17657, t17658, t17677, t17682, t2251, t2258, t3367, t3720, t3723, t44273, t44276, t44278, t44280, t44283, t44286, t44289, t44510, t44517, t44952, t5287, t5297);
        let (t56997, t56999, t57005, t57021, t57026) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3103(t13148, t56878, t1121, t3601, t606, t17728, t460, t489, t17261, t17373, t12772, t17639, t3625);
        let t57047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3104(t12772, t17645, t3625, t1284, t17288, t3624, t12917, t17401, t1121, t11231, t12732, t12855, t12862, t12876, t13046, t16714, t16756, t17353, t17454, t17456, t17609, t17654, t17655, t21017, t2258, t3588, t3591, t3604, t3720, t3723, t44225, t44291, t44293, t44326, t44484, t45764, t471, t5330, t5331, t5332, t5335, t56997, t56999, t57005, t57021, t57026, t606);
    (t56878, t56903, t56907, t56932, t56981, t56985, t56999, t57005, t57047)
}
