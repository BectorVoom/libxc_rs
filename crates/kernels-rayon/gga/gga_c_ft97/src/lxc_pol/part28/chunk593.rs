//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 593/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk593(t1564: f64, t25569: f64, t22922: f64, t925: f64, t1285: f64, t3051: f64, t3052: f64, t5502: f64, t15593: f64, t2: f64, t4: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25570 = t1564 * t25569;
    let t25574 = t1564 * t22922 * t925;
    let t25577 = t1285 * t3051;
    let t25579 = t1564 * t5502 * t3052;
    let t25582 = t15593 * t2;
    let t25583 = t25582 * t4;
    let t25584 = t25583 * t26;
    (t25570, t25574, t25577, t25579, t25582, t25584)
}
