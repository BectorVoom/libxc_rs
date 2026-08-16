//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1063/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1063(t1531: f64, t466: f64, t5146: f64, t4871: f64, t4877: f64, t1502: f64, t1612: f64, t16540: f64, t4915: f64, t555: f64, t5137: f64, t546: f64) -> (f64, f64, f64, f64, f64) {
    let t16599 = 0.1301229756036208781e0_f64 * t1531 * t466 * t5146;
    let t16600 = t4871 * t4877;
    let t16603 = 1.0_f64 / t1502 / t1612;
    let t16607 = 0.12304822629859687989e5_f64 * t555 * t16603 * t16540 * t4915;
    let t16612 = 480.0_f64 * t5137 * t546;
    (t16599, t16600, t16603, t16607, t16612)
}
