//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 847/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk847(t5854: f64, t733: f64, t5845: f64, t743: f64, t5848: f64, t1971: f64, t2471: f64, t1976: f64, t2475: f64, t1968: f64, t2466: f64, t13589: f64, t5839: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17151 = t733 * t5854;
    let t17174 = 0.4705225e-4_f64 * t743 * t5845;
    let t17175 = t743 * t5848;
    let t17197 = t2471 * t1971;
    let t17199 = t2475 * t1976;
    let t17201 = t2466 * t1968;
    let t17203 = t13589 * t5839;
    (t17151, t17174, t17175, t17197, t17199, t17201, t17203)
}
