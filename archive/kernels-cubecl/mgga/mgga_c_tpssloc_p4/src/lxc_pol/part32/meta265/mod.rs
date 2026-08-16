//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta265 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1194;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1195;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1196;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1197;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1198;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1199;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1200;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta265<F: Float>(t2132: F, t52: F, t2136: F, t6729: F, t1184: F, t460: F, t2147: F, t478: F, t2131: F, t6739: F, t2133: F, t461: F, t1009: F, t1209: F, t1215: F, t68: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7315, t7316, t7319) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1194::<F>(t2132, t52, t2136, t6729, t1184, t460);
        let t7320 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1195::<F>(t2147, t478);
        let t7321 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1196::<F>(t7319, t7320);
        let t7324 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1197::<F>(t2131, t6739);
        let (t7325, t7326) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1198::<F>(t2133, t461, t7324);
        let t7327 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1199::<F>(t1009, t1209);
        let t7328 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1200::<F>(t478, t7327);
        let (t7330, t7331) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1201::<F>(t1215, t68, t475, t7328);
    (t7315, t7316, t7319, t7320, t7321, t7324, t7325, t7326, t7327, t7328, t7330, t7331)
}
