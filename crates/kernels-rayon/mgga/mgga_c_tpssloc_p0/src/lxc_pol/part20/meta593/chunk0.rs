//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2172/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2172(t44178: f64, t11176: f64, t1147: f64, t3368: f64, t3400: f64, t3375: f64, t11292: f64, t1143: f64, t3324: f64, t3331: f64, t1124: f64, t11419: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44179 = 1.0_f64 / t44178;
    let t44183 = t11176 * t1147;
    let t44188 = t3368 * t3400;
    let t44202 = t3368 * t3375;
    let t44205 = t1143 * t11292;
    let t44211 = t3324 * t3331;
    let t44214 = t1124 * t11419;
    (t44179, t44183, t44188, t44202, t44205, t44211, t44214)
}
