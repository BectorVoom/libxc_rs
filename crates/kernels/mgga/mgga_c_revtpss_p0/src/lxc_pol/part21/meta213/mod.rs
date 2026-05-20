//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta213 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1281;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1282;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1283;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1284;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1285;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta213<F: Float>(t1150: F, t5104: F, t1131: F, t1732: F, t3435: F, t1149: F, t3433: F, t3358: F, t3439: F, t5044: F, t5049: F, t5054: F, t5058: F, t1160: F, t1737: F, t1168: F, t1745: F, t3415: F, t3459: F, t3466: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t5096: F, t5099: F, t5102: F, t1169: F, t1744: F, t3479: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t5105 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1281::<F>(t1150, t5104);
        let (t5107, t5108) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1282::<F>(t1131, t5105, t1732, t3435);
        let (t5109, t5111, t5117, t5120, t5125) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1283::<F>(t1149, t5108, t3433, t3358, t3439, t5044, t5049, t5054, t5058, t1160, t1737, t1168, t1745);
        let t5142 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1284::<F>(t3358, t3415, t3459, t3466, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096, t5099, t5102);
        let t5143 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1285::<F>(t1169, t5142);
        let t5146 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1286::<F>(t1744, t3479);
    (t5105, t5107, t5108, t5109, t5111, t5117, t5120, t5125, t5142, t5143, t5146)
}
