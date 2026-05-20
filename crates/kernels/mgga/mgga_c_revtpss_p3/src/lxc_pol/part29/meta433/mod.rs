//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta433 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1605;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1606;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1607;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1608;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1609;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta433<F: Float>(t17539: F, t5296: F, t1042: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F, t12268: F, t3617: F, t15936: F, t3708: F, t5265: F, t13392: F, t5302: F, t1252: F, t1261: F, t12956: F, t17525: F, t17529: F, t17536: F, t3591: F, t3606: F, t3613: F, t3711: F, t5293: F, t5299: F, t1260: F, t5326: F, t17376: F, t3599: F, t17482: F, t3604: F, t3720: F, t3372: F, t5277: F, t12855: F, t12964: F, t12979: F, t12985: F, t12996: F, t3620: F, t3640: F, t3714: F, t5381: F, t5391: F, t3368: F, t3704: F, t5274: F, t1774: F, t3588: F, t1250: F, t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t12999: F, t13012: F, t13015: F, t13018: F, t3631: F, t3647: F, t3718: F, t5279: F, t5304: F, t12916: F, t5353: F, t5347: F, t3568: F, t471: F, t5351: F, t1781: F, t697: F, t1222: F, t5284: F, t73: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17541, t17546, t17547, t17552, t17556) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1605::<F>(t17539, t5296, t1042, t3172, t5286, t1247, t3707, t5292, t12268, t3617, t15936, t3708, t5265);
        let t17561 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1606::<F>(t13392, t5302, t1042, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t17552, t17556, t3591, t3606, t3613, t3711, t5293, t5299);
        let t17587 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1607::<F>(t1260, t5326, t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
        let (t17589, t17593, t17600, t17602, t17605, t17608) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1608::<F>(t3368, t5277, t1042, t3704, t5274, t1774, t3588, t1250, t3720, t1285, t17395, t1032, t5216);
        let t17614 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1609::<F>(t1246, t17608, t1252, t12956, t12999, t13012, t13015, t13018, t17589, t17593, t17602, t17605, t3631, t3647, t3711, t3718, t5279, t5304);
        let (t17619, t17622, t17625, t17629, t17633) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1610::<F>(t12916, t5353, t3718, t5347, t3568, t471, t5351, t3720, t1781, t697, t1222, t5284, t73);
    (t17561, t17587, t17600, t17614, t17619, t17622, t17625, t17629, t17633)
}
