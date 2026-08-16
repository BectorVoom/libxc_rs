//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2373/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373(t291: f64, t48702: f64, t48722: f64, t10709: f64, t4483: f64, t10661: f64, t10662: f64, t1557: f64, t10817: f64, t14382: f64, t14385: f64, t42143: f64) -> (f64, f64, f64, f64, f64) {
    let t48725 = 0.621814e-1_f64 * (t48702 + t48722) * t291;
    let t48727 = 0.35089341735807877242e1_f64 * t4483 * t10709;
    let t48730 = 24.0_f64 * t10661 * t1557 * t10662;
    let t48732 = 6.0_f64 * t10817 * t14382;
    let t48734 = 0.28947563097646563121e3_f64 * t42143 * t14385;
    (t48725, t48727, t48730, t48732, t48734)
}
