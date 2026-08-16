//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1141/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1141(t59: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t116: f64, t557: f64, t1314: f64, t9534: f64, t9223: f64, t120: f64, t212: f64, t22815: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40344 = t59 * t598;
    let t40347 = 0.11265432098765432099e0_f64 * t40344 * t535 * t795;
    let t40350 = 0.14979423868312757201e0_f64 * t39933 * t535 * t215;
    let t40353 = t557 * t116;
    let t40369 = t9534 * t1314 * t116;
    let t40394 = t59 * t9223;
    let t40399 = t116 * t67 * t22815 * t120 * t212;
    (t40344, t40347, t40350, t40353, t40369, t40394, t40399)
}
