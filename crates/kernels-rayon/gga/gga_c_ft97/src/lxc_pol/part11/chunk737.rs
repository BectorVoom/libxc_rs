//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 737/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk737(t2405: f64, t2594: f64, t773: f64, t2544: f64, t681: f64, t89: f64, t2399: f64, t756: f64, t2567: f64, t754: f64, t2569: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9993 = t2594 * t773 * t2405;
    let t9997 = t89 * t681 * t2544;
    let t10000 = t89 * t2399 * t756;
    let t10002 = t754 * t2567;
    let t10003 = t10002 * t2569;
    let t10004 = t242 * t10003;
    (t9993, t9997, t10000, t10002, t10003, t10004)
}
