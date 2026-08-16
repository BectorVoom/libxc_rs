//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta833 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3115;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3117;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3118;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3119;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3120;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta833<F: Float>(t57273: F, t17170: F, t73: F, t13014: F, t5373: F, t12998: F, t1222: F, t140: F, t17404: F, t12941: F, t5293: F, t5274: F, t1263: F, t16750: F, t1012: F, t1042: F, t1122: F, t1225: F, t12787: F, t12836: F, t12956: F, t13002: F, t13008: F, t17502: F, t17605: F, t17736: F, t17737: F, t3625: F, t3626: F, t3629: F, t3711: F, t49889: F, t5046: F, t57083: F, t57257: F, t57258: F, t57265: F, t57271: F, t17547: F, t3704: F, t17609: F, t12901: F, t17525: F, t1261: F, t17551: F, t3172: F, t3588: F, t5333: F, t44250: F, t5406: F, t12773: F, t17448: F, t12916: F, t17780: F, t5331: F, t1260: F, t45385: F, t12640: F, t17728: F, t489: F, t12257: F, t12712: F, t12781: F, t12832: F, t12872: F, t13045: F, t13099: F, t1715: F, t17347: F, t17351: F, t17584: F, t17602: F, t17688: F, t17709: F, t17710: F, t17739: F, t1774: F, t17747: F, t17753: F, t20945: F, t21049: F, t3603: F, t3720: F, t44501: F, t471: F, t3153: F, t12744: F, t17350: F, t3781: F, t5219: F, t5330: F, t17743: F, t3718: F, t1469: F, t11243: F, t1802: F, t1244: F, t13036: F, t12881: F, t5391: F, t16720: F, t17471: F, t11231: F, t12784: F, t12812: F, t12855: F, t12922: F, t13065: F, t16719: F, t16756: F, t17355: F, t17505: F, t17600: F, t17633: F, t17640: F, t17649: F, t17654: F, t17703: F, t17742: F, t21028: F, t21119: F, t44508: F, t44769: F, t5312: F, t5340: F, t5348: F, t56205: F, t57005: F, t17755: F, t12800: F, t5378: F, t17769: F, t3647: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57274, t57275, t57290, t57292, t57295, t57297, t57299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3115::<F>(t57273, t17170, t73, t13014, t5373, t12998, t1222, t140, t17404, t12941, t5293, t5274);
        let t57308 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116::<F>(t1263, t16750, t1012, t1042, t1122, t1222, t1225, t12787, t12836, t12956, t13002, t13008, t17502, t17605, t17736, t17737, t3625, t3626, t3629, t3711, t49889, t5046, t5373, t57083, t57257, t57258, t57265, t57271, t57274, t57275, t57290, t57292, t57295, t57297, t57299);
        let (t57314, t57316, t57318, t57321, t57325, t57331) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3117::<F>(t17547, t3704, t17609, t12901, t17525, t1261, t17551, t3172, t3588, t5333, t3625, t44250, t5406);
        let t57370 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3118::<F>(t12773, t17448, t12916, t17780, t5331, t1260, t45385, t12640, t17728, t489, t1042, t12257, t12712, t12781, t12832, t12872, t12956, t13045, t13099, t1715, t17347, t17351, t17584, t17602, t17605, t17688, t17709, t17710, t17739, t1774, t17747, t17753, t20945, t21049, t3603, t3626, t3711, t3720, t44501, t471, t57314, t57316, t57318, t57321, t57325, t57331);
        let (t57373, t57378, t57382, t57386, t57394) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3119::<F>(t17170, t3153, t12744, t17350, t3781, t5219, t5330, t12916, t17743, t3718, t1469, t3588);
        let (t57403, t57433) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3120::<F>(t11243, t1802, t1244, t13036, t12881, t5391, t1222, t16720, t17471, t11231, t12784, t12787, t12812, t12855, t12922, t13065, t16719, t16756, t17351, t17355, t17505, t17600, t17633, t17640, t17649, t17654, t17703, t17742, t21028, t21119, t3718, t3720, t44508, t44769, t5312, t5331, t5333, t5340, t5348, t56205, t57005, t57373, t57378, t57382, t57386, t57394);
        let (t57435, t57449, t57451, t57464, t57465) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3121::<F>(t12916, t17753, t17755, t12800, t5378, t17769, t3647, t1235, t371, t5318, t676, t225, t56331);
    (t57275, t57308, t57325, t57370, t57373, t57403, t57433, t57435, t57449, t57451, t57464, t57465)
}
