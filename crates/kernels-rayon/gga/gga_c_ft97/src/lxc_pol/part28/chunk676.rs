//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 676/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk676(t1348: f64, t3051: f64, t1969: f64, t3052: f64, t5773: f64, t3450: f64, t9432: f64, t6579: f64, t92: f64, t23408: f64, t925: f64, t1882: f64, t6632: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26809 = t1348 * t3051;
    let t26811 = t1969 * t5773 * t3052;
    let t26815 = t9432 * t5773 * t3450;
    let t26817 = t6579 * t92;
    let t26822 = t23408 * t925;
    let t26823 = t1969 * t26822;
    let t26826 = t1882 * t6632;
    (t26809, t26811, t26815, t26817, t26822, t26823, t26826)
}
