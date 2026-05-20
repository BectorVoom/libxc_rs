//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3100;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3101;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3102;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3103;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta831<F: Float>(t3623: F, t53739: F, t13127: F, t12865: F, t3746: F, t13396: F, t5405: F, t13392: F, t17672: F, t4181: F, t17677: F, t17682: F, t1042: F, t1261: F, t1264: F, t12712: F, t17351: F, t17353: F, t17644: F, t17654: F, t17693: F, t17694: F, t17696: F, t17799: F, t17800: F, t1797: F, t247: F, t3629: F, t44248: F, t44252: F, t44264: F, t44267: F, t44270: F, t44585: F, t5302: F, t54450: F, t56232: F, t1214: F, t3611: F, t12831: F, t17395: F, t12702: F, t17350: F, t1263: F, t372: F, t5284: F, t1250: F, t12809: F, t12862: F, t12866: F, t13069: F, t16696: F, t16756: F, t17396: F, t17482: F, t17512: F, t17649: F, t17657: F, t17658: F, t2251: F, t2258: F, t3367: F, t3720: F, t3723: F, t44273: F, t44276: F, t44278: F, t44280: F, t44283: F, t44286: F, t44289: F, t44510: F, t44517: F, t44952: F, t5287: F, t5297: F, t13148: F, t1121: F, t3601: F, t606: F, t17728: F, t460: F, t489: F, t17261: F, t17373: F, t12772: F, t17639: F, t3625: F, t17645: F, t1284: F, t17288: F, t3624: F, t12917: F, t17401: F, t11231: F, t12732: F, t12855: F, t12876: F, t13046: F, t16714: F, t17454: F, t17456: F, t17609: F, t17655: F, t21017: F, t3588: F, t3591: F, t3604: F, t44225: F, t44291: F, t44293: F, t44326: F, t44484: F, t45764: F, t471: F, t5330: F, t5331: F, t5332: F, t5335: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t56878, t56879, t56888, t56891, t56895, t56899, t56903, t56907) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3100::<F>(t3623, t53739, t13127, t12865, t3746, t13396, t5405, t13392, t17672, t4181, t17677, t17682);
        let t56932 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3101::<F>(t1042, t1261, t1264, t12712, t17351, t17353, t17644, t17654, t17693, t17694, t17696, t17799, t17800, t1797, t247, t3629, t44248, t44252, t44264, t44267, t44270, t44585, t5302, t54450, t56232, t56879, t56888, t56891, t56895, t56899, t56903, t56907);
        let (t56981, t56985) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3102::<F>(t1214, t3611, t12831, t17395, t12702, t17350, t1263, t372, t5284, t1250, t12809, t12862, t12866, t13069, t16696, t16756, t17353, t17396, t17482, t17512, t17649, t17654, t17657, t17658, t17677, t17682, t2251, t2258, t3367, t3720, t3723, t44273, t44276, t44278, t44280, t44283, t44286, t44289, t44510, t44517, t44952, t5287, t5297);
        let (t56997, t56999, t57005, t57021, t57026) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3103::<F>(t13148, t56878, t1121, t3601, t606, t17728, t460, t489, t17261, t17373, t12772, t17639, t3625);
        let t57047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3104::<F>(t12772, t17645, t3625, t1284, t17288, t3624, t12917, t17401, t1121, t11231, t12732, t12855, t12862, t12876, t13046, t16714, t16756, t17353, t17454, t17456, t17609, t17654, t17655, t21017, t2258, t3588, t3591, t3604, t3720, t3723, t44225, t44291, t44293, t44326, t44484, t45764, t471, t5330, t5331, t5332, t5335, t56997, t56999, t57005, t57021, t57026, t606);
    (t56878, t56903, t56907, t56932, t56981, t56985, t56999, t57005, t57047)
}
