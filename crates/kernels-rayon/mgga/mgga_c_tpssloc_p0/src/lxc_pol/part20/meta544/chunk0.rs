//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2085/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2085(t10033: f64, t41011: f64, t2632: f64, t9957: f64, t9638: f64, t9653: f64, t9623: f64, t10003: f64, t10009: f64, t2617: f64, t9600: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41012 = t41011 * t10033;
    let t41014 = t2632 * t9957;
    let t41025 = t9638 * t9653;
    let t41031 = t9638 * t9623;
    let t41048 = t9638 * t10003;
    let t41050 = t9638 * t10009;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    (t41012, t41014, t41025, t41031, t41048, t41050, t41052, t41053)
}
