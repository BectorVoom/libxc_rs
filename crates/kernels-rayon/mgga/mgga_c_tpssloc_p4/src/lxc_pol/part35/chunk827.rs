//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 827/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk827(t116: f64, t131: f64, t9537: f64, t207: f64, t9534: f64, t2559: f64, t786: f64, t2566: f64, t2570: f64, t792: f64, t154: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9538 = t116 * t131 * t9537;
    let t9540 = 0.13888888888888888889e-3_f64 * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    let t9546 = t2566 * t786;
    let t9549 = t792 * t2570;
    let t9558 = t154 * t845;
    (t9538, t9540, t9541, t9546, t9549, t9558)
}
