//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 552/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk552(t205: f64, t2570: f64, t210: f64, t214: f64, t2379: f64, t786: f64, t792: f64, t118: f64, t776: f64, t794: f64, t2553: f64, t59: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2571 = t205 * t2570;
    let t2573 = t210 * t214 * t2379;
    let t2576 = t792 * t786;
    let t2578 = t118 * t794 * t776;
    let t2579 = t2576 * t2578;
    let t2582 = t210 * t214 * t2553;
    let t2585 = t59 * t835;
    (t2571, t2573, t2576, t2578, t2579, t2582, t2585)
}
