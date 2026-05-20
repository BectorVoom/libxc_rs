//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1339;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta211<F: Float>(t3417: F, t5047: F, t141: F, t1145: F, t5052: F, t5056: F, t3358: F, t3402: F, t3414: F, t3415: F, t5044: F, t5049: F, t5054: F, t5058: F, t5072: F, t5080: F, t5088: F, t5090: F, t5093: F, t1150: F, t1131: F, t1732: F, t3435: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5095, t5096, t5098, t5099, t5101, t5102, t5104) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1338::<F>(t3417, t5047, t141, t1145, t5052, t5056, t3358, t3402, t3414, t3415, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093);
        let t5105 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1339::<F>(t1150, t5104);
        let (t5107, t5108) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1340::<F>(t1131, t5105, t1732, t3435);
    (t5095, t5096, t5098, t5099, t5101, t5102, t5104, t5105, t5107, t5108)
}
