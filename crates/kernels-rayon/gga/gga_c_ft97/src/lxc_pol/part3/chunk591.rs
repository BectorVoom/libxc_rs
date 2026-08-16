//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 591/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk591(t2348: f64, t4917: f64, t2345: f64, t89: f64, t1091: f64, t1131: f64, t2354: f64, t446: f64, t2361: f64, t666: f64, t4635: f64, t669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4918 = t2348 * t4917;
    let t4920 = t89 * t2345 * t4918;
    let t4922 = t1091 * t1131;
    let t4923 = t2354 * t4922;
    let t4924 = t446 * t4923;
    let t4926 = t2361 * t4917;
    let t4928 = t89 * t666 * t4926;
    let t4930 = t669 * t4635;
    (t4918, t4920, t4922, t4923, t4924, t4926, t4928, t4930)
}
