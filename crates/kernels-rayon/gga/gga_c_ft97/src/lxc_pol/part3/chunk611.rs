//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 611/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk611(t241: f64, t258: f64, t5132: f64, t2518: f64, t3688: f64, t3710: f64, t4920: f64, t4924: f64, t4928: f64, t4932: f64, t4937: f64, t5056: f64, t5094: f64, t5122: f64) -> (f64, f64) {
    let t5134 = t241 * t5132 * t258;
    let t5147 = -t5094 / 4.0_f64 + t5122 / 2.0_f64 + t2518 + 2.0_f64 / 9.0_f64 * t3688 + 2.0_f64 / 3.0_f64 * t3710 - 2.0_f64 / 9.0_f64 * t4920 + 2.0_f64 / 3.0_f64 * t4924 + 2.0_f64 / 3.0_f64 * t4928 - t4932 / 3.0_f64 + 2.0_f64 * t4937 - t5056;
    (t5134, t5147)
}
