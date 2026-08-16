//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1174/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1174(t11778: f64, t121: f64, t1229: f64, t204: f64, t11604: f64, t496: f64, t68: f64, t107: f64, t9576: f64, t106: f64, t9364: f64, t35761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45268 = t121 * t11778;
    let t45293 = t204 * t1229;
    let t45349 = 1.0_f64 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45421 = 2618.0_f64 / 81.0_f64 * t9576 * t107;
    let t45435 = 1.0_f64 / t9364 / t106;
    let t45460 = 1.0_f64 / t35761;
    (t45268, t45293, t45350, t45421, t45435, t45460)
}
