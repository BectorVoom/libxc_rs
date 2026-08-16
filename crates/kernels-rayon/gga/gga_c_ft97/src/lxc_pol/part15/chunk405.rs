//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 405/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk405(t1063: f64, t5: f64, t1068: f64, t2253: f64, t179: f64, t422: f64, t71: f64, t1576: f64, t171: f64, t11: f64, t41: f64, t1075: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3601 = t5 * t1063;
    let t3611 = t2253 * t1068;
    let t3613 = t422 * t179;
    let t3621 = t71 * t179;
    let t3626 = 1.0_f64 / t171 / t1576;
    let t3627 = t11 * t3626;
    let t3628 = t41 * t3627;
    let t3633 = t2253 * t1075;
    (t3601, t3611, t3613, t3621, t3626, t3628, t3633)
}
