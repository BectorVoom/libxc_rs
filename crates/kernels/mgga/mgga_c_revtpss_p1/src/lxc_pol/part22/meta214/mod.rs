//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1352;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta214<F: Float>(t1198: F, t5192: F, t1765: F, t3531: F, t1756: F, t3495: F, t1189: F, t1196: F, t1179: F, t1188: F, t5180: F, t3520: F, t1187: F, t3523: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5194, t5196, t5197, t5198, t5200, t5202, t5204, t5205) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1352::<F>(t1198, t5192, t1765, t3531, t1756, t3495, t1189, t1196, t1179, t1188, t5180, t3520);
        let t5206 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1353::<F>(t1187, t3523);
    (t5194, t5196, t5197, t5198, t5200, t5202, t5204, t5205, t5206)
}
