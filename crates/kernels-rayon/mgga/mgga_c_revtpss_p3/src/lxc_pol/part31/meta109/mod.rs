//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk650;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta109(t675: f64, t738: f64, t182: f64, t737: f64, t177: f64, t2492: f64, t745: f64, t2514: f64, t2491: f64, t2495: f64, t123: f64, t173: f64, t186: f64, t2434: f64, t2522: f64, t2531: f64, t2537: f64, t2539: f64, t2549: f64, t2554: f64, t2557: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t268: f64, t724: f64, t731: f64, t739: f64, t746: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2591, t2596, t2597, t2598, t2601, t2604, t2605, t2608) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk650(t675, t738, t182, t737, t177, t2492, t745, t2514, t2491, t2495, t123, t173, t186, t2434, t2522, t2531, t2537, t2539, t2549, t2554, t2557, t2562, t2569, t2579, t2587, t268, t724, t731, t739, t746);
        let t2609 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk651(t162, t2608);
    (t2591, t2596, t2597, t2598, t2601, t2604, t2605, t2608, t2609)
}
