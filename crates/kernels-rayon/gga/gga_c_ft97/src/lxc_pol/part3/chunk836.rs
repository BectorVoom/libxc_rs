//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 836/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk836(t17006: f64, t2222: f64, t2221: f64, t4431: f64, t609: f64, t2211: f64, t2210: f64, t2178: f64, t4724: f64, t379: f64, t160: f64, t4668: f64) -> (f64, f64, f64, f64) {
    let t17007 = t2222 * t17006;
    let t17008 = t2221 * t17007;
    let t17011 = t4431 * t609;
    let t17012 = t2211 * t17011;
    let t17013 = t2210 * t17012;
    let t17016 = t2178 * t4724;
    let t17017 = t17016 * t379;
    let t17018 = t2210 * t17017;
    let t17021 = t160 * t4668;
    (t17008, t17013, t17018, t17021)
}
