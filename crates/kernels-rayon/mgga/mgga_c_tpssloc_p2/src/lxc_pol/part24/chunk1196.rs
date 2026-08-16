//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1196/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1196(t344: f64, t6729: f64, t6740: f64, t3008: f64, t343: f64, t6734: f64, t3103: f64, t6755: f64, t3120: f64, t360: f64, t68: f64, t6744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23488 = t6729 * t344;
    let t23489 = t6740 * t23488;
    let t23494 = t3008 * t343;
    let t23495 = t23494 * t6734;
    let t23500 = t6755 * t3103;
    let t23503 = t3120 * t68 * t360;
    let t23504 = t6744 * t23503;
    (t23488, t23489, t23494, t23495, t23500, t23503, t23504)
}
