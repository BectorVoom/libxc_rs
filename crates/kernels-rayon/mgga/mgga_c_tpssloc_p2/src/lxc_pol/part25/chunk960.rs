//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 960/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk960(t12168: f64, t1343: f64, t820: f64, t3799: f64, t3858: f64, t12267: f64, t1340: f64, t120: f64, t3850: f64, t3805: f64, t3807: f64, t3719: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12392 = t1343 * t820 * t12168;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    let t12402 = t120 * t3850;
    let t12404 = t3805 * t12402 * t3807;
    let t12407 = t550 * t3719;
    (t12392, t12395, t12397, t12402, t12404, t12407)
}
