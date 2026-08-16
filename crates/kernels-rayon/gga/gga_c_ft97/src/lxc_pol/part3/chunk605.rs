//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 605/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk605(t5053: f64, t676: f64, t27: f64, t89: f64, t2335: f64, t3942: f64, t3947: f64, t4920: f64, t4924: f64, t4928: f64, t4932: f64, t4937: f64) -> (f64, f64, f64) {
    let t5054 = t676 * t5053;
    let t5056 = t89 * t27 * t5054;
    let t5058 = t2335 + t3942 + t3947 - t4920 / 27.0_f64 + t4924 / 9.0_f64 + t4928 / 9.0_f64 - t4932 / 18.0_f64 + t4937 / 3.0_f64 - t5056 / 6.0_f64;
    (t5054, t5056, t5058)
}
