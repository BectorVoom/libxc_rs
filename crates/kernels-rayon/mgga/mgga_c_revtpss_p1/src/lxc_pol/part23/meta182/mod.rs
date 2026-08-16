//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta182 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1081;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1082;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1083;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1084;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1085;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta182(t1169: f64, t5142: f64, t1744: f64, t3479: f64, t1168: f64, t3358: f64, t3483: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t448: f64, t1179: f64, t1749: f64, t1187: f64, t1757: f64, t3415: f64, t3503: f64, t3510: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64, t5099: f64, t5102: f64, t1188: f64, t1756: f64, t3523: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t1745: f64, t3447: f64, t3452: f64, t3477: f64, t3491: f64, t3496: f64, t3521: f64, t435: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5117: f64, t5120: f64, t5125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5143, t5146, t5147, t5155) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1081(t1169, t5142, t1744, t3479, t1168, t3358, t3483, t5044, t5049, t5054, t5058);
        let (t5156, t5158) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1082(t448, t5155, t1179, t1749);
        let (t5163, t5180) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1083(t1187, t1757, t3358, t3415, t3503, t3510, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096, t5099, t5102);
        let t5181 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1084(t1188, t5180);
        let t5184 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1085(t1756, t3523);
        let (t5185, t5188) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1086(t1187, t5184, t1161, t1170, t1180, t1189, t1745, t1757, t3447, t3452, t3477, t3491, t3496, t3521, t435, t5062, t5065, t5067, t5070, t5107, t5111, t5117, t5120, t5125, t5143, t5147, t5156, t5158, t5163, t5181);
    (t5143, t5146, t5147, t5155, t5156, t5158, t5163, t5180, t5181, t5184, t5185, t5188)
}
