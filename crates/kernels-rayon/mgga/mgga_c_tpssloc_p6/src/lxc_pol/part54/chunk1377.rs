//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1377/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1377(t22986: f64, t23270: f64, t31337: f64, t4119: f64, t33458: f64, t6579: f64, t114792: f64, t118791: f64, t118792: f64, t118802: f64, t121413: f64, t121419: f64, t121426: f64, t121429: f64, t121431: f64, t1911: f64, t26679: f64, t2718: f64, t31311: f64, t4268: f64, t855: f64) -> f64 {
    let t121435 = t22986 * t23270 * t31337 * t4119;
    let t121437 = t6579 * t33458;
    let t121440 = 0.16449340668482264365e-1_f64 * t121413 + 2.0_f64 * t4268 * t31311 - 0.3289868133696452873e-1_f64 * t121419 + 2.0_f64 * t855 * t2718 * t26679 * t1911 + 0.16449340668482264365e-1_f64 * t121426 + 0.16449340668482264365e-1_f64 * t121429 + 0.19190897446562641759e-1_f64 * t121431 + t118791 + t118792 + t118802 + 0.16449340668482264365e-1_f64 * t121435 - 0.38381794893125283518e-1_f64 * t121437 + 0.41123351671205660912e-2_f64 * t114792;
    t121440
}
