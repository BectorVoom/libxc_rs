//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 594/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk594(t3704: f64, t4044: f64, t89: f64, t1213: f64, t375: f64, t1212: f64, t668: f64, t505: f64, t2665: f64, t446: f64, t2680: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4046 = t89 * t3704 * t4044;
    let t4049 = t89 * t375 * t1213;
    let t4051 = t1212 * t668;
    let t4052 = t4051 * t505;
    let t4053 = t2665 * t4052;
    let t4054 = t446 * t4053;
    let t4056 = t2680 * t1212;
    let t4057 = t4056 * t824;
    (t4046, t4049, t4051, t4052, t4053, t4054, t4056, t4057)
}
