//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 536/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk536(t187: f64, t5: f64, t2302: f64, t2306: f64, t2313: f64, t2336: f64, t2344: f64, t2346: f64, t2350: f64, t2357: f64, t2368: f64, t2376: f64, t2705: f64, t864: f64, t867: f64) -> (f64, f64) {
    let t2709 = t187 * t5;
    let t2718 = -t2302 - t2306 - t2313 + t2336 + t2344 + t187 * t2705 + 0.24415406715670879921e-3_f64 * t864 * t2346 + 0.10843580882781524214e-1_f64 * t2709 * t2350 + 0.11696446794910408142e1_f64 * t867 * t2357 - 0.58482233974552040708e0_f64 * t867 * t2368 - 0.17315755899375863299e2_f64 * t867 * t2376;
    (t2709, t2718)
}
