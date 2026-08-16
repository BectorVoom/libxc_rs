//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 382/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk382(t109: f64, t111: f64, t1275: f64, t1279: f64, t1286: f64, t1735: f64, t1742: f64, t1763: f64, t1767: f64, t1832: f64, t260: f64, t271: f64, t427: f64, t436: f64, t437: f64, t695: f64) -> f64 {
    let t1835 = -0.11281315546296296296e-3_f64 * t109 * t1275 * t271 + 0.1e-22_f64 * t436 * t1279 * t271 - 0.67687893277777777778e-3_f64 * t109 * t427 * t695 + 0.50765919958333333334e-3_f64 * t1286 * t1735 + 0.50765919958333333334e-3_f64 * t436 * t437 * t695 + 0.10153183991666666667e-2_f64 * t109 * t111 * t1742 - 0.50765919958333333334e-3_f64 * t109 * t111 * t1763 - 4.0_f64 * t1767 - 4.0_f64 * t260 * t1832;
    t1835
}
