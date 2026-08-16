//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta833 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3115;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3117;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3118;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3119;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3120;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta833(t57273: f64, t17170: f64, t73: f64, t13014: f64, t5373: f64, t12998: f64, t1222: f64, t140: f64, t17404: f64, t12941: f64, t5293: f64, t5274: f64, t1263: f64, t16750: f64, t1012: f64, t1042: f64, t1122: f64, t1225: f64, t12787: f64, t12836: f64, t12956: f64, t13002: f64, t13008: f64, t17502: f64, t17605: f64, t17736: f64, t17737: f64, t3625: f64, t3626: f64, t3629: f64, t3711: f64, t49889: f64, t5046: f64, t57083: f64, t57257: f64, t57258: f64, t57265: f64, t57271: f64, t17547: f64, t3704: f64, t17609: f64, t12901: f64, t17525: f64, t1261: f64, t17551: f64, t3172: f64, t3588: f64, t5333: f64, t44250: f64, t5406: f64, t12773: f64, t17448: f64, t12916: f64, t17780: f64, t5331: f64, t1260: f64, t45385: f64, t12640: f64, t17728: f64, t489: f64, t12257: f64, t12712: f64, t12781: f64, t12832: f64, t12872: f64, t13045: f64, t13099: f64, t1715: f64, t17347: f64, t17351: f64, t17584: f64, t17602: f64, t17688: f64, t17709: f64, t17710: f64, t17739: f64, t1774: f64, t17747: f64, t17753: f64, t20945: f64, t21049: f64, t3603: f64, t3720: f64, t44501: f64, t471: f64, t3153: f64, t12744: f64, t17350: f64, t3781: f64, t5219: f64, t5330: f64, t17743: f64, t3718: f64, t1469: f64, t11243: f64, t1802: f64, t1244: f64, t13036: f64, t12881: f64, t5391: f64, t16720: f64, t17471: f64, t11231: f64, t12784: f64, t12812: f64, t12855: f64, t12922: f64, t13065: f64, t16719: f64, t16756: f64, t17355: f64, t17505: f64, t17600: f64, t17633: f64, t17640: f64, t17649: f64, t17654: f64, t17703: f64, t17742: f64, t21028: f64, t21119: f64, t44508: f64, t44769: f64, t5312: f64, t5340: f64, t5348: f64, t56205: f64, t57005: f64, t17755: f64, t12800: f64, t5378: f64, t17769: f64, t3647: f64, t1235: f64, t371: f64, t5318: f64, t676: f64, t225: f64, t56331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57274, t57275, t57290, t57292, t57295, t57297, t57299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3115(t57273, t17170, t73, t13014, t5373, t12998, t1222, t140, t17404, t12941, t5293, t5274);
        let t57308 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116(t1263, t16750, t1012, t1042, t1122, t1222, t1225, t12787, t12836, t12956, t13002, t13008, t17502, t17605, t17736, t17737, t3625, t3626, t3629, t3711, t49889, t5046, t5373, t57083, t57257, t57258, t57265, t57271, t57274, t57275, t57290, t57292, t57295, t57297, t57299);
        let (t57314, t57316, t57318, t57321, t57325, t57331) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3117(t17547, t3704, t17609, t12901, t17525, t1261, t17551, t3172, t3588, t5333, t3625, t44250, t5406);
        let t57370 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3118(t12773, t17448, t12916, t17780, t5331, t1260, t45385, t12640, t17728, t489, t1042, t12257, t12712, t12781, t12832, t12872, t12956, t13045, t13099, t1715, t17347, t17351, t17584, t17602, t17605, t17688, t17709, t17710, t17739, t1774, t17747, t17753, t20945, t21049, t3603, t3626, t3711, t3720, t44501, t471, t57314, t57316, t57318, t57321, t57325, t57331);
        let (t57373, t57378, t57382, t57386, t57394) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3119(t17170, t3153, t12744, t17350, t3781, t5219, t5330, t12916, t17743, t3718, t1469, t3588);
        let (t57403, t57433) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3120(t11243, t1802, t1244, t13036, t12881, t5391, t1222, t16720, t17471, t11231, t12784, t12787, t12812, t12855, t12922, t13065, t16719, t16756, t17351, t17355, t17505, t17600, t17633, t17640, t17649, t17654, t17703, t17742, t21028, t21119, t3718, t3720, t44508, t44769, t5312, t5331, t5333, t5340, t5348, t56205, t57005, t57373, t57378, t57382, t57386, t57394);
        let (t57435, t57449, t57451, t57464, t57465) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3121(t12916, t17753, t17755, t12800, t5378, t17769, t3647, t1235, t371, t5318, t676, t225, t56331);
    (t57275, t57308, t57325, t57370, t57373, t57403, t57433, t57435, t57449, t57451, t57464, t57465)
}
