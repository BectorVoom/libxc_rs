//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 562/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk562(t43: f64, t50: f64, t40: f64, t4580: f64, t1933: f64, t4561: f64, t4565: f64, t607: f64, t1940: f64, t4570: f64, t4573: f64, t611: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t4581 = t40 * t4580;
    let t4587 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t1933 * t4561 + 2.0_f64 / 3.0_f64 * t607 * t4565);
    let t4593 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1940 * t4570 + 2.0_f64 / 3.0_f64 * t611 * t4573);
    let t4595 = t4587 / 2.0_f64 + t4593 / 2.0_f64;
    (t4581, t4595)
}
