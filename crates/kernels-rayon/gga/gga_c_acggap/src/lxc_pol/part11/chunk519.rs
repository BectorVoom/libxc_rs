//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 519/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk519(t180: f64, t864: f64, t407: f64, t3088: f64, t113: f64, t2607: f64, t2690: f64, t4: f64) -> (f64, f64) {
    let t3089 = t180 * t864;
    let t3090 = t3089 * t407;
    let t3091 = t3088 * t3090;
    let t3101 = -0.12962962962962962963e0_f64 * t4 * t2607 * t113 - 0.71338703703703703708e-1_f64 * t2690;
    (t3091, t3101)
}
