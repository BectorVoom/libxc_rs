//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1607;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1610;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1613;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta428(t1032: f64, t1246: f64, t12690: f64, t12904: f64, t3708: f64, t11262: f64, t1247: f64, t3590: f64, t3610: f64, t3612: f64, t13069: f64, t3704: f64, t12941: f64, t12269: f64, t12273: f64, t1252: f64, t12781: f64, t12784: f64, t12787: f64, t12789: f64, t3625: f64, t3626: f64, t3714: f64, t44248: f64, t44252: f64, t44260: f64, t44264: f64, t5405: f64, t12948: f64, t13058: f64, t12937: f64, t3172: f64, t3711: f64, t13080: f64, t5384: f64, t1231: f64, t12898: f64, t3651: f64, t3655: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t459: f64, t1256: f64, t12890: f64, t3588: f64, t482: f64, t1222: f64, t3693: f64, t697: f64, t13021: f64, t140: f64, t12256: f64, t3698: f64, t1012: f64, t1042: f64, t12800: f64, t12816: f64, t225: f64, t3600: f64, t3604: f64, t3620: f64, t3647: f64, t3692: f64, t39443: f64, t39449: f64, t480: f64, t484: f64, t3362: f64, t414: f64, t66: f64, t42859: f64, t460: f64, t42865: f64, t479: f64, t1244: f64, t3601: f64, t42871: f64, t471: f64, t1261: f64, t12884: f64, t247: f64, t13085: f64, t12277: f64, t3634: f64, t13089: f64, t1122: f64, t12629: f64, t1263: f64, t12926: f64, t13076: f64, t17202: f64, t17344: f64, t2251: f64, t3363: f64, t3568: f64, t3584: f64, t3617: f64, t3618: f64, t43767: f64, t43869: f64, t44205: f64, t5268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44267, t44270, t44273, t44276, t44278) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1607(t1032, t1246, t12690, t12904, t3708, t11262, t1247, t3590, t3610, t3612, t13069, t3704);
        let t44282 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608(t12941, t3708, t12269, t12273, t1252, t12781, t12784, t12787, t12789, t3625, t3626, t3714, t44248, t44252, t44260, t44264, t44267, t44270, t44273, t44276, t44278, t5405);
        let (t44283, t44286, t44289, t44291, t44293, t44306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609(t12948, t13058, t12937, t3172, t3711, t13080, t5384, t1231, t12898, t3651, t3655, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44319 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1610(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44321, t44326, t44332, t44333, t44343, t44346) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611(t44306, t44319, t459, t1256, t12890, t3588, t482, t1222, t3693, t697, t13021, t140);
        let t44353 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612(t12256, t3698, t1012, t1042, t1222, t12800, t12816, t225, t3600, t3604, t3620, t3647, t3692, t39443, t39449, t44283, t44286, t44289, t44291, t44293, t44321, t44326, t44333, t44343, t44346, t480, t484);
        let (t44362, t44372, t44373, t44375, t44376, t44377, t44378) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1613(t3362, t414, t66, t42859, t460, t42865, t479, t1244, t3601, t482, t42871, t471);
        let t44417 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614(t12269, t1261, t12884, t247, t13085, t3647, t12277, t3634, t13089, t12273, t1042, t1122, t12629, t1263, t12926, t13076, t17202, t17344, t2251, t3363, t3568, t3584, t3617, t3618, t3708, t3711, t43767, t43869, t44205, t44362, t44375, t44377, t44378, t5268, t5384);
    (t44282, t44321, t44332, t44333, t44353, t44372, t44373, t44376, t44377, t44417)
}
