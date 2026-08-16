//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1393/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1393(t1162: f64, t2367: f64, t9045: f64, t3244: f64, t9142: f64, t9197: f64, t9205: f64, t1150: f64, t3224: f64, t7274: f64, t3200: f64, t3212: f64, t3213: f64) -> (f64, f64, f64, f64, f64) {
    let t27755 = t1162 * t2367 * t9045;
    let t27758 = t3244 * t9142 * t9197;
    let t27761 = t1162 * t2367 * t9205;
    let t27768 = t1150 * t7274 * t3224;
    let t27771 = t3212 * t3200 * t3213;
    (t27755, t27758, t27761, t27768, t27771)
}
