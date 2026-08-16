//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 900/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk900(t2570: f64, t792: f64, t154: f64, t845: f64, t205: f64, t59: f64, t8705: f64, t207: f64, t215: f64, t782: f64, t2690: f64, t2588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9549 = t792 * t2570;
    let t9558 = t154 * t845;
    let t9559 = t205 * t9558;
    let t9569 = t59 * t8705;
    let t9572 = 0.28086419753086419752e-1_f64 * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9576 = t59 * t2690;
    let t9577 = t9576 * t154;
    let t9579 = 0.99999999999999999997e-2_f64 * t9577 * t2588;
    (t9549, t9558, t9559, t9569, t9572, t9573, t9576, t9577, t9579)
}
