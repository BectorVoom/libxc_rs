//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1003/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1003(t7: f64, t7274: f64, t1179: f64, t7281: f64, t2680: f64, t3: f64, t1793: f64, t1796: f64, t1808: f64, t224: f64, t3619: f64, t3622: f64, t461: f64, t8636: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9338 = 32.0_f64 * t7274;
    let t9340 = t7281 * t1179;
    let t9343 = t2680 * t3;
    let t9353 = piecewise3(t8, 0.0_f64, -8.0_f64 / 27.0_f64 * t9340 * t1808 + 16.0_f64 / 9.0_f64 * t9343 * t8636 + 4.0_f64 / 9.0_f64 * t3619 * t1796 + 8.0_f64 / 3.0_f64 * t224 * t1793 - 8.0_f64 * t3622 * t461);
    (t9338, t9340, t9353)
}
