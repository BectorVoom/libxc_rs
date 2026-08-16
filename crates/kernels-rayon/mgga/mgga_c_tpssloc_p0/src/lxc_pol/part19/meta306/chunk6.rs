//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1099/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099(t1864: f64, t2244: f64, t2245: f64, t2250: f64, t2283: f64, t2304: f64, t31: f64, t33: f64, t39103: f64, t39110: f64, t39213: f64, t607: f64, t628: f64, t642: f64, t65: f64, t6509: f64, t67: f64, t80: f64, t9247: f64, t9248: f64, t9251: f64, t9258: f64, t9259: f64, t9260: f64) -> f64 {
    let t39217 = -t39103 * t65 * t80 / 4.0_f64 - t607 * t628 * t67 * t9248 - t9247 * t6509 * t2250 - t9247 * t1864 * t9258 / 3.0_f64 - t31 * t39110 * t65 * t80 / 12.0_f64 - t9259 * t628 * t80 / 3.0_f64 - t9260 * t642 / 3.0_f64 - t2244 * t2283 * t80 / 2.0_f64 - t9251 * t642 - t2245 * t2304 / 2.0_f64 + t33 * t39213 * t80 / 24.0_f64;
    t39217
}
