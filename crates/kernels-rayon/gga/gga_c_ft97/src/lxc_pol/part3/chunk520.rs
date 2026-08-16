//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 520/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk520(t1212: f64, t668: f64, t505: f64, t2665: f64, t446: f64, t2680: f64, t824: f64, t193: f64, t89: f64, t284: f64, t811: f64, t1197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4051 = t1212 * t668;
    let t4052 = t4051 * t505;
    let t4053 = t2665 * t4052;
    let t4054 = t446 * t4053;
    let t4056 = t2680 * t1212;
    let t4057 = t4056 * t824;
    let t4059 = t89 * t193 * t4057;
    let t4061 = t811 * t284;
    let t4062 = t4061 * t1197;
    (t4052, t4053, t4054, t4056, t4057, t4059, t4061, t4062)
}
