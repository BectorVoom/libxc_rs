//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1116/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116(t12088: f64, t2535: f64, t2504: f64, t2368: f64, t746: f64, t1294: f64, t268: f64, t676: f64, t9478: f64, t9482: f64) -> (f64, f64, f64, f64, f64) {
    let t39387 = t12088 * t2535;
    let t39388 = 0.35089341735807877242e1_f64 * t39387;
    let t39389 = t2504 * t2504;
    let t39391 = t2368 * t39389 * t746;
    let t39393 = 0.35089341735807877242e1_f64 * t1294 * t39391;
    let t39397 = 0.3684616320282908548e2_f64 * t268 * t676 * t9478 * t9482;
    (t39388, t39389, t39391, t39393, t39397)
}
