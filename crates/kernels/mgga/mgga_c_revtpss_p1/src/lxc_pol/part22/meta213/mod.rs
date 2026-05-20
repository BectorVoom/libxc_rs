//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta213 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1347;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1348;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1349;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1350;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta213<F: Float>(t1187: F, t1757: F, t3358: F, t3415: F, t3503: F, t3510: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F, t1188: F, t1756: F, t3523: F, t1161: F, t1170: F, t1180: F, t1189: F, t1745: F, t3447: F, t3452: F, t3477: F, t3491: F, t3496: F, t3521: F, t435: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5117: F, t5120: F, t5125: F, t5143: F, t5147: F, t5156: F, t5158: F, t300: F, t1749: F) -> (F, F, F, F, F, F, F, F) {
        let (t5163, t5180) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1347::<F>(t1187, t1757, t3358, t3415, t3503, t3510, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096, t5099, t5102);
        let t5181 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1348::<F>(t1188, t5180);
        let t5184 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1349::<F>(t1756, t3523);
        let (t5185, t5188) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1350::<F>(t1187, t5184, t1161, t1170, t1180, t1189, t1745, t1757, t3447, t3452, t3477, t3491, t3496, t3521, t435, t5062, t5065, t5067, t5070, t5107, t5111, t5117, t5120, t5125, t5143, t5147, t5156, t5158, t5163, t5181);
        let (t5189, t5191, t5192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1351::<F>(t300, t5188, t5156, t1749);
    (t5163, t5180, t5181, t5184, t5185, t5189, t5191, t5192)
}
