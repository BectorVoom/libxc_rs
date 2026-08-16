//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 670/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk670(t2371: f64, t665: f64, t2: f64, t740: f64, t8282: f64, t9802: f64, t249: f64, t3051: f64, t1771: f64, t745: f64, t241: f64, t9567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9895 = t665 * t2371;
    let t9896 = t9895 * t2;
    let t9907 = t8282 * t740;
    let t9916 = t9802 * t2;
    let t9935 = 28.0_f64 / 27.0_f64 * t3051 * t249;
    let t9936 = t1771 * t745;
    let t9952 = t9567 * t241;
    (t9895, t9896, t9907, t9916, t9935, t9936, t9952)
}
