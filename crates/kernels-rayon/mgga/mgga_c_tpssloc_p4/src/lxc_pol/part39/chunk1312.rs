//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1312/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1312(t1453: f64, t2358: f64, t4072: f64, t649: f64, t12813: f64, t88: f64, t1458: f64, t2311: f64, t89: f64, t626: f64, t9365: f64, t45435: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86598 = t1453 * t2358;
    let t90370 = t649 * t4072;
    let t90375 = t88 * t12813;
    let t90381 = t2311 * t1458;
    let t91753 = t89 * t12813;
    let t110075 = t626 * t9365;
    let t110082 = t64 * t45435;
    (t86598, t90370, t90375, t90381, t91753, t110075, t110082)
}
