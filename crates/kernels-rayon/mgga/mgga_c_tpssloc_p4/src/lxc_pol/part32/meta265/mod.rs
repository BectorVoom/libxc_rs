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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1194;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1195;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1196;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1197;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1198;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1199;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1200;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta265(t2132: f64, t52: f64, t2136: f64, t6729: f64, t1184: f64, t460: f64, t2147: f64, t478: f64, t2131: f64, t6739: f64, t2133: f64, t461: f64, t1009: f64, t1209: f64, t1215: f64, t68: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7315, t7316, t7319) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1194(t2132, t52, t2136, t6729, t1184, t460);
        let t7320 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1195(t2147, t478);
        let t7321 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1196(t7319, t7320);
        let t7324 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1197(t2131, t6739);
        let (t7325, t7326) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1198(t2133, t461, t7324);
        let t7327 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1199(t1009, t1209);
        let t7328 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1200(t478, t7327);
        let (t7330, t7331) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1201(t1215, t68, t475, t7328);
    (t7315, t7316, t7319, t7320, t7321, t7324, t7325, t7326, t7327, t7328, t7330, t7331)
}
