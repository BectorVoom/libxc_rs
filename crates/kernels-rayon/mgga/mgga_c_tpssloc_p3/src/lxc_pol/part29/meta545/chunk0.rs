//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1942/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1942(t2122: f64, t27381: f64, t1186: f64, t4733: f64, t7286: f64, t7285: f64, t1716: f64, t24638: f64, t1760: f64, t7391: f64, t3598: f64, t24574: f64, t8003: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27382 = t2122 * t27381;
    let t27383 = t1186 * t27382;
    let t27388 = t7286 * t4733;
    let t27389 = t7285 * t27388;
    let t27392 = t1716 * t24638;
    let t27395 = t7391 * t1760;
    let t27396 = t3598 * t27395;
    let t27401 = t24574 * t8003;
    (t27382, t27383, t27388, t27389, t27392, t27396, t27401)
}
