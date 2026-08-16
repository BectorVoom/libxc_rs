//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1645/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1645(t2663: f64, t3814: f64, t3681: f64, t67: f64, t758: f64, t1294: f64, t9905: f64, t9892: f64, t3826: f64, t588: f64, t3684: f64, t9467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12097 = t3814 * t2663;
    let t12098 = 0.73245789224026180216e-3_f64 * t12097;
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    let t12101 = 0.54934341918019635162e-3_f64 * t12100;
    let t12103 = 0.35089341735807877242e1_f64 * t1294 * t9905;
    let t12105 = 0.51947577317044391277e2_f64 * t1294 * t9892;
    let t12106 = t588 * t3826;
    let t12107 = 24.0_f64 * t12106;
    let t12109 = 0.21687162600603479684e-1_f64 * t3684 * t9467;
    (t12097, t12098, t12099, t12100, t12101, t12103, t12105, t12107, t12109)
}
