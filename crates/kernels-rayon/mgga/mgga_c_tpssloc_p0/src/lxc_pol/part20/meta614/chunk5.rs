//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2209/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209(t25: f64, t28: f64, t88: f64, t9416: f64, t1406: f64, t9238: f64, t16: f64, t39031: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t45814 = t88 * t9416;
    let t45844 = t1406 * t9238;
    let t45869 = 12.0_f64 * t16;
    let t45870 = 24.0_f64 * t39031;
    let t45872 = piecewise5(t26, 0.0_f64, t29, 0.0_f64, -t45869 + t45870);
    (t45814, t45844, t45872)
}
