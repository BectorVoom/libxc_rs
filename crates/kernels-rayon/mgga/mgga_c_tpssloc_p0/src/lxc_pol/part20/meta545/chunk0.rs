//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2086/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2086(t2707: f64, t9993: f64, t2642: f64, t9612: f64, t9638: f64, t9649: f64, t2678: f64, t828: f64, t786: f64, t9569: f64, t805: f64, t2610: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41055 = t9993 * t2707;
    let t41063 = t9612 * t2642;
    let t41066 = t9638 * t9649;
    let t41078 = t2678 * t828;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    let t41086 = t9541 * t2610;
    (t41055, t41063, t41066, t41078, t41083, t41084, t41086)
}
