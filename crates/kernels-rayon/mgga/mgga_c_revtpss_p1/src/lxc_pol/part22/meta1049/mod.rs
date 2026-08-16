//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1049 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3686;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3687;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3688;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3689;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3690;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3691;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3692;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3693;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1049(t21094: f64, t3172: f64, t5384: f64, t17361: f64, t5274: f64, t5261: f64, t5390: f64, t12915: f64, t20703: f64, t247: f64, t17373: f64, t21203: f64, t1230: f64, t21271: f64, t1266: f64, t12800: f64, t17763: f64, t1808: f64, t21242: f64, t21272: f64, t3640: f64, t3644: f64, t5397: f64, t57187: f64, t6683: f64, t1263: f64, t21082: f64, t3584: f64, t5819: f64, t1042: f64, t1122: f64, t1261: f64, t12956: f64, t17202: f64, t17235: f64, t20811: f64, t20868: f64, t20907: f64, t20914: f64, t3647: f64, t3711: f64, t5268: f64, t5270: f64, t5279: f64, t5304: f64, t57053: f64, t60834: f64, t60838: f64, t65370: f64, t17544: f64, t5293: f64, t21275: f64, t17769: f64, t5381: f64, t5391: f64, t1247: f64, t20902: f64, t1234: f64, t17265: f64, t17502: f64, t17505: f64, t17569: f64, t20809: f64, t3372: f64, t3568: f64, t3714: f64, t5296: f64, t5302: f64, t56713: f64, t5825: f64, t17209: f64, t12855: f64, t12916: f64, t21120: f64, t21093: f64, t372: f64, t13046: f64, t17214: f64, t3588: f64, t3601: f64, t3604: f64, t3630: f64, t3720: f64, t44500: f64, t44521: f64, t56718: f64, t56720: f64, t56726: f64, t56728: f64, t56734: f64, t56739: f64, t56742: f64, t6688: f64, t6628: f64, t19680: f64, t5405: f64, t20823: f64, t21233: f64, t17451: f64, t17605: f64, t12784: f64, t12866: f64, t17381: f64, t17656: f64, t17693: f64, t17799: f64, t21022: f64, t3618: f64, t56758: f64, t56785: f64, t56787: f64, t56790: f64, t56793: f64, t56997: f64, t57710: f64, t68395: f64, t20824: f64, t20879: f64, t1214: f64, t17232: f64, t17412: f64, t17541: f64, t17552: f64, t18281: f64, t21184: f64, t56796: f64, t56798: f64, t56812: f64, t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t17225: f64, t21183: f64, t20875: f64, t12809: f64, t16771: f64, t17199: f64, t17204: f64, t17344: f64, t17550: f64, t1789: f64, t21028: f64, t21257: f64, t57136: f64, t65947: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69698, t69700, t69710, t69719, t69721) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3686(t21094, t3172, t5384, t17361, t5274, t5261, t5390, t12915, t20703, t247, t17373, t21203);
        let t69728 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3687(t1230, t21271, t1266, t12800, t17763, t1808, t21242, t21272, t3640, t3644, t5397, t57187, t6683, t69698, t69700, t69710, t69719, t69721);
        let (t69763, t69770) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3688(t1263, t21082, t3584, t5819, t1042, t1122, t1261, t12956, t17202, t17235, t17763, t20811, t20868, t20907, t20914, t3647, t3711, t5268, t5270, t5279, t5304, t57053, t60834, t60838, t65370);
        let (t69773, t69783, t69787, t69789, t69793, t69795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3689(t17544, t5293, t17373, t21275, t17769, t5381, t5391, t1247, t20902, t3172, t1234, t21271);
        let t69805 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3690(t1042, t1261, t17265, t17502, t17505, t17569, t20809, t21203, t3372, t3568, t3711, t3714, t5296, t5302, t5384, t56713, t5825, t60838, t69773, t69783, t69787, t69789, t69793, t69795);
        let t69836 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3691(t17209, t17505, t12855, t12916, t21120, t21093, t372, t13046, t17214, t21203, t3588, t3601, t3604, t3630, t3720, t44500, t44521, t56718, t56720, t56726, t56728, t56734, t56739, t56742, t6688);
        let (t69839, t69844, t69848, t69868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3692(t1263, t372, t6628, t19680, t5405, t20823, t21233, t3647, t17451, t17605, t1261, t12784, t12866, t13046, t17381, t17656, t17693, t17799, t21022, t247, t3618, t56758, t56785, t56787, t56790, t56793, t56997, t57710, t68395);
        let (t69875, t69901) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3693(t3568, t5819, t17209, t17569, t20824, t3172, t3711, t20879, t1042, t1214, t12956, t17232, t17412, t17505, t17541, t17552, t18281, t21184, t5296, t5302, t5304, t5381, t5384, t56796, t56798, t56812);
        let t69943 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3694(t1260, t20850, t11262, t3600, t6630, t17225, t5391, t21183, t3172, t3711, t20875, t1042, t1261, t12809, t16771, t17199, t17204, t17344, t17412, t17550, t1789, t21028, t21257, t3584, t3714, t3720, t5270, t5296, t57136, t5825, t65370, t65947);
    (t69728, t69763, t69770, t69805, t69836, t69839, t69844, t69848, t69868, t69875, t69901, t69943)
}
