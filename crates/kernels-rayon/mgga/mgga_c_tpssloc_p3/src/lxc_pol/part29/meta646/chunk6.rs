//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2138/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2138(t81633: f64, t13453: f64, t1499: f64, t23151: f64, t25261: f64, t25281: f64, t2684: f64, t4291: f64, t81623: f64, t81630: f64, t81642: f64, t81653: f64, t87527: f64, t87531: f64, t87534: f64, t87536: f64, t87538: f64, t87541: f64, t87545: f64, t87547: f64, t87554: f64) -> f64 {
    let t87559 = 0.25587863262083522346e0_f64 * t81633;
    let t87562 = -0.16449340668482264365e-1_f64 * t87527 - 0.6579736267392905746e-1_f64 * t87531 + t87534 + t87536 - 0.82246703342411321825e-2_f64 * t87538 + 0.3289868133696452873e-1_f64 * t87541 - t87545 - t87547 + 4.0_f64 * t13453 * t25281 - t4291 * t25261 * t2684 - 0.16449340668482264365e-1_f64 * t87554 + t1499 * t23151 + 0.76763589786250567036e-1_f64 * t81623 + 0.82246703342411321824e-2_f64 * t81630 - t87559 - 0.24674011002723396547e-1_f64 * t81642 - 0.16449340668482264365e-1_f64 * t81653;
    t87562
}
