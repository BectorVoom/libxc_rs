//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1293/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1293(t116: f64, t131: f64, t9537: f64, t207: f64, t9534: f64, t2559: f64, t786: f64, t789: f64, t2566: f64, t2578: f64, t2570: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9538 = t116 * t131 * t9537;
    let t9540 = 0.13888888888888888889e-3_f64 * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    let t9542 = t9541 * t789;
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    let t9549 = t792 * t2570;
    (t9538, t9540, t9541, t9542, t9546, t9547, t9549)
}
