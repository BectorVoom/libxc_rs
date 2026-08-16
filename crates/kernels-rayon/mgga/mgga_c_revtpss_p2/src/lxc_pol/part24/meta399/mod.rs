//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1332;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1333;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta399(t123: f64, t147: f64, t39500: f64, t164: f64, t215: f64, t2491: f64, t2531: f64, t2536: f64, t2539: f64, t2596: f64, t2598: f64, t2601: f64, t2605: f64, t268: f64, t39967: f64, t40056: f64, t40059: f64, t40067: f64, t40072: f64, t40076: f64, t675: f64, t723: f64, t731: f64, t738: f64, t746: f64, t793: f64, t9367: f64, t9417: f64, t9432: f64, t9435: f64, t9447: f64, t9461: f64, t9469: f64, t9476: f64, t9481: f64, t9488: f64, t9525: f64, t9529: f64, t9533: f64, t9537: f64, t39913: f64, t39957: f64, t40007: f64, t158: f64, t162: f64, t2492: f64, t9507: f64, t760: f64, t9385: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t40079 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1332(t123, t147, t39500);
        let t40080 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1333(t164, t215, t2491, t2531, t2536, t2539, t2596, t2598, t2601, t2605, t268, t39967, t40056, t40059, t40067, t40072, t40076, t40079, t675, t723, t731, t738, t746, t793, t9367, t9417, t9432, t9435, t9447, t9461, t9469, t9476, t9481, t9488, t9525, t9529, t9533, t9537);
        let (t40082, t40084, t40086, t40088, t40097) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1334(t39913, t39957, t40007, t40080, t158, t162, t2492, t9417, t9507, t760, t2596, t746, t9385);
    (t40079, t40082, t40084, t40086, t40088, t40097)
}
