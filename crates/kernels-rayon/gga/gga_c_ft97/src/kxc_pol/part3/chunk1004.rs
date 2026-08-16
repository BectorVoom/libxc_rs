//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1004/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1004(t18712: f64, t4265: f64, t2881: f64, t4140: f64, t4139: f64, t19460: f64, t10479: f64, t10485: f64, t19465: f64, t1212: f64, t4311: f64, t840: f64) -> (f64, f64, f64, f64, f64) {
    let t19534 = t4265 * t18712;
    let t19535 = t2881 * t19534;
    let t19538 = t4140 * t18712;
    let t19539 = t4139 * t19538;
    let t19542 = t4140 * t19460;
    let t19543 = t10479 * t19542;
    let t19546 = t10485 * t19465;
    let t19547 = t4139 * t19546;
    let t19551 = t840 * t4311 * t1212;
    (t19535, t19539, t19543, t19547, t19551)
}
