//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1429;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1430;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1431;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1432;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta395(t17539: f64, t5296: f64, t1042: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64, t15936: f64, t3708: f64, t5265: f64, t13392: f64, t5302: f64, t1252: f64, t1261: f64, t12956: f64, t17525: f64, t17529: f64, t17536: f64, t3591: f64, t3606: f64, t3613: f64, t3711: f64, t5293: f64, t5299: f64, t1260: f64, t5326: f64, t17376: f64, t3599: f64, t17482: f64, t3604: f64, t3720: f64, t3372: f64, t5277: f64, t12855: f64, t12964: f64, t12979: f64, t12985: f64, t12996: f64, t3620: f64, t3640: f64, t3714: f64, t5381: f64, t5391: f64, t3368: f64, t3704: f64, t5274: f64, t1774: f64, t3588: f64, t1250: f64, t1285: f64, t17395: f64, t1032: f64, t5216: f64, t1246: f64, t12999: f64, t13012: f64, t13015: f64, t13018: f64, t3631: f64, t3647: f64, t3718: f64, t5279: f64, t5304: f64, t12916: f64, t5353: f64, t5347: f64, t3568: f64, t471: f64, t5351: f64, t1781: f64, t697: f64, t1222: f64, t5284: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17541, t17546, t17547, t17552, t17556) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1428(t17539, t5296, t1042, t3172, t5286, t1247, t3707, t5292, t12268, t3617, t15936, t3708, t5265);
        let t17561 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1429(t13392, t5302, t1042, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t17552, t17556, t3591, t3606, t3613, t3711, t5293, t5299);
        let t17587 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1430(t1260, t5326, t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
        let (t17589, t17593, t17600, t17602, t17605, t17608) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1431(t3368, t5277, t1042, t3704, t5274, t1774, t3588, t1250, t3720, t1285, t17395, t1032, t5216);
        let t17614 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1432(t1246, t17608, t1252, t12956, t12999, t13012, t13015, t13018, t17589, t17593, t17602, t17605, t3631, t3647, t3711, t3718, t5279, t5304);
        let (t17619, t17622, t17625, t17629, t17633) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1433(t12916, t5353, t3718, t5347, t3568, t471, t5351, t3720, t1781, t697, t1222, t5284, t73);
    (t17561, t17587, t17600, t17614, t17619, t17622, t17625, t17629, t17633)
}
