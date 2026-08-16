//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 270/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk270(t519: f64, t920: f64, t356: f64, t89: f64, t528: f64, t929: f64, t126: f64, t120: f64, t534: f64) -> (f64, f64, f64, f64, f64) {
    let t998 = t519 * t920;
    let t1000 = t89 * t356 * t998;
    let t1002 = t528 * t929;
    let t1005 = t929 * t126;
    let t1008 = 0.23410285231011484e0_f64 * t1002 * t120 - 0.532971647967385935e-1_f64 * t534 * t1005;
    (t998, t1000, t1002, t1005, t1008)
}
