//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2516/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2516(t44938: f64, t45971: f64, t48140: f64, t45192: f64, t2403: f64, t4775: f64, t14795: f64, t699: f64, t14798: f64, t136: f64, t3297: f64, t50959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51034 = t48140 * t44938 * t45971;
    let t51037 = t48140 * t45192 * t45971;
    let t51039 = t2403 * t4775;
    let t51040 = 0.5519e0_f64 * t51039;
    let t51041 = t699 * t14795;
    let t51043 = t699 * t14798;
    let t51046 = t136 * t3297 * t50959;
    (t51034, t51037, t51039, t51040, t51041, t51043, t51046)
}
