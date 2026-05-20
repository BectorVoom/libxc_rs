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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1607;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1610;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1613;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta428<F: Float>(t1032: F, t1246: F, t12690: F, t12904: F, t3708: F, t11262: F, t1247: F, t3590: F, t3610: F, t3612: F, t13069: F, t3704: F, t12941: F, t12269: F, t12273: F, t1252: F, t12781: F, t12784: F, t12787: F, t12789: F, t3625: F, t3626: F, t3714: F, t44248: F, t44252: F, t44260: F, t44264: F, t5405: F, t12948: F, t13058: F, t12937: F, t3172: F, t3711: F, t13080: F, t5384: F, t1231: F, t12898: F, t3651: F, t3655: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F, t43854: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t459: F, t1256: F, t12890: F, t3588: F, t482: F, t1222: F, t3693: F, t697: F, t13021: F, t140: F, t12256: F, t3698: F, t1012: F, t1042: F, t12800: F, t12816: F, t225: F, t3600: F, t3604: F, t3620: F, t3647: F, t3692: F, t39443: F, t39449: F, t480: F, t484: F, t3362: F, t414: F, t66: F, t42859: F, t460: F, t42865: F, t479: F, t1244: F, t3601: F, t42871: F, t471: F, t1261: F, t12884: F, t247: F, t13085: F, t12277: F, t3634: F, t13089: F, t1122: F, t12629: F, t1263: F, t12926: F, t13076: F, t17202: F, t17344: F, t2251: F, t3363: F, t3568: F, t3584: F, t3617: F, t3618: F, t43767: F, t43869: F, t44205: F, t5268: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44267, t44270, t44273, t44276, t44278) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1607::<F>(t1032, t1246, t12690, t12904, t3708, t11262, t1247, t3590, t3610, t3612, t13069, t3704);
        let t44282 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1608::<F>(t12941, t3708, t12269, t12273, t1252, t12781, t12784, t12787, t12789, t3625, t3626, t3714, t44248, t44252, t44260, t44264, t44267, t44270, t44273, t44276, t44278, t5405);
        let (t44283, t44286, t44289, t44291, t44293, t44306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609::<F>(t12948, t13058, t12937, t3172, t3711, t13080, t5384, t1231, t12898, t3651, t3655, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44319 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1610::<F>(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44321, t44326, t44332, t44333, t44343, t44346) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611::<F>(t44306, t44319, t459, t1256, t12890, t3588, t482, t1222, t3693, t697, t13021, t140);
        let t44353 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1612::<F>(t12256, t3698, t1012, t1042, t1222, t12800, t12816, t225, t3600, t3604, t3620, t3647, t3692, t39443, t39449, t44283, t44286, t44289, t44291, t44293, t44321, t44326, t44333, t44343, t44346, t480, t484);
        let (t44362, t44372, t44373, t44375, t44376, t44377, t44378) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1613::<F>(t3362, t414, t66, t42859, t460, t42865, t479, t1244, t3601, t482, t42871, t471);
        let t44417 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614::<F>(t12269, t1261, t12884, t247, t13085, t3647, t12277, t3634, t13089, t12273, t1042, t1122, t12629, t1263, t12926, t13076, t17202, t17344, t2251, t3363, t3568, t3584, t3617, t3618, t3708, t3711, t43767, t43869, t44205, t44362, t44375, t44377, t44378, t5268, t5384);
    (t44282, t44321, t44332, t44333, t44353, t44372, t44373, t44376, t44377, t44417)
}
