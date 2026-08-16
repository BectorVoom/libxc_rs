//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk501;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk502;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk503;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk504;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta85(t675: f64, t738: f64, t182: f64, t737: f64, t177: f64, t2492: f64, t745: f64, t2514: f64, t2491: f64, t2495: f64, t123: f64, t173: f64, t186: f64, t2434: f64, t2522: f64, t2531: f64, t2537: f64, t2539: f64, t2549: f64, t2554: f64, t2557: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t268: f64, t724: f64, t731: f64, t739: f64, t746: f64, t162: f64, t158: f64, t157: f64, t37: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2591, t2595, t2596) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk501(t675, t738, t182, t737);
        let (t2597, t2598) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk502(t177, t2596, t2492, t745);
        let (t2601, t2604, t2605, t2608) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk503(t2514, t745, t177, t2491, t2492, t2495, t123, t173, t186, t2434, t2522, t2531, t2537, t2539, t2549, t2554, t2557, t2562, t2569, t2579, t2587, t2591, t2597, t2598, t268, t724, t731, t739, t746);
        let t2609 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk504(t162, t2608);
        let (t2610, t2611) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk505(t158, t2609, t157, t37);
    (t2591, t2595, t2596, t2597, t2598, t2601, t2604, t2605, t2608, t2609, t2610, t2611)
}
