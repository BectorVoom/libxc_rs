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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1049<F: Float>(t21094: F, t3172: F, t5384: F, t17361: F, t5274: F, t5261: F, t5390: F, t12915: F, t20703: F, t247: F, t17373: F, t21203: F, t1230: F, t21271: F, t1266: F, t12800: F, t17763: F, t1808: F, t21242: F, t21272: F, t3640: F, t3644: F, t5397: F, t57187: F, t6683: F, t1263: F, t21082: F, t3584: F, t5819: F, t1042: F, t1122: F, t1261: F, t12956: F, t17202: F, t17235: F, t20811: F, t20868: F, t20907: F, t20914: F, t3647: F, t3711: F, t5268: F, t5270: F, t5279: F, t5304: F, t57053: F, t60834: F, t60838: F, t65370: F, t17544: F, t5293: F, t21275: F, t17769: F, t5381: F, t5391: F, t1247: F, t20902: F, t1234: F, t17265: F, t17502: F, t17505: F, t17569: F, t20809: F, t3372: F, t3568: F, t3714: F, t5296: F, t5302: F, t56713: F, t5825: F, t17209: F, t12855: F, t12916: F, t21120: F, t21093: F, t372: F, t13046: F, t17214: F, t3588: F, t3601: F, t3604: F, t3630: F, t3720: F, t44500: F, t44521: F, t56718: F, t56720: F, t56726: F, t56728: F, t56734: F, t56739: F, t56742: F, t6688: F, t6628: F, t19680: F, t5405: F, t20823: F, t21233: F, t17451: F, t17605: F, t12784: F, t12866: F, t17381: F, t17656: F, t17693: F, t17799: F, t21022: F, t3618: F, t56758: F, t56785: F, t56787: F, t56790: F, t56793: F, t56997: F, t57710: F, t68395: F, t20824: F, t20879: F, t1214: F, t17232: F, t17412: F, t17541: F, t17552: F, t18281: F, t21184: F, t56796: F, t56798: F, t56812: F, t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t17225: F, t21183: F, t20875: F, t12809: F, t16771: F, t17199: F, t17204: F, t17344: F, t17550: F, t1789: F, t21028: F, t21257: F, t57136: F, t65947: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69698, t69700, t69710, t69719, t69721) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3686::<F>(t21094, t3172, t5384, t17361, t5274, t5261, t5390, t12915, t20703, t247, t17373, t21203);
        let t69728 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3687::<F>(t1230, t21271, t1266, t12800, t17763, t1808, t21242, t21272, t3640, t3644, t5397, t57187, t6683, t69698, t69700, t69710, t69719, t69721);
        let (t69763, t69770) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3688::<F>(t1263, t21082, t3584, t5819, t1042, t1122, t1261, t12956, t17202, t17235, t17763, t20811, t20868, t20907, t20914, t3647, t3711, t5268, t5270, t5279, t5304, t57053, t60834, t60838, t65370);
        let (t69773, t69783, t69787, t69789, t69793, t69795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3689::<F>(t17544, t5293, t17373, t21275, t17769, t5381, t5391, t1247, t20902, t3172, t1234, t21271);
        let t69805 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3690::<F>(t1042, t1261, t17265, t17502, t17505, t17569, t20809, t21203, t3372, t3568, t3711, t3714, t5296, t5302, t5384, t56713, t5825, t60838, t69773, t69783, t69787, t69789, t69793, t69795);
        let t69836 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3691::<F>(t17209, t17505, t12855, t12916, t21120, t21093, t372, t13046, t17214, t21203, t3588, t3601, t3604, t3630, t3720, t44500, t44521, t56718, t56720, t56726, t56728, t56734, t56739, t56742, t6688);
        let (t69839, t69844, t69848, t69868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3692::<F>(t1263, t372, t6628, t19680, t5405, t20823, t21233, t3647, t17451, t17605, t1261, t12784, t12866, t13046, t17381, t17656, t17693, t17799, t21022, t247, t3618, t56758, t56785, t56787, t56790, t56793, t56997, t57710, t68395);
        let (t69875, t69901) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3693::<F>(t3568, t5819, t17209, t17569, t20824, t3172, t3711, t20879, t1042, t1214, t12956, t17232, t17412, t17505, t17541, t17552, t18281, t21184, t5296, t5302, t5304, t5381, t5384, t56796, t56798, t56812);
        let t69943 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3694::<F>(t1260, t20850, t11262, t3600, t6630, t17225, t5391, t21183, t3172, t3711, t20875, t1042, t1261, t12809, t16771, t17199, t17204, t17344, t17412, t17550, t1789, t21028, t21257, t3584, t3714, t3720, t5270, t5296, t57136, t5825, t65370, t65947);
    (t69728, t69763, t69770, t69805, t69836, t69839, t69844, t69848, t69868, t69875, t69901, t69943)
}
