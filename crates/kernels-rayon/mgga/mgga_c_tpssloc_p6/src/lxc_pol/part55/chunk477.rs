//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 477/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk477(t2576: f64, t2578: f64, t59: f64, t835: f64, t154: f64, t116: f64, t206: f64, t212: f64, t225: f64, t799: f64) -> (f64, f64, f64, f64, f64) {
    let t2579 = t2576 * t2578;
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3_f64 * t2586 * t2588;
    let t2597 = t799 * t225;
    (t2579, t2586, t2587, t2590, t2597)
}
