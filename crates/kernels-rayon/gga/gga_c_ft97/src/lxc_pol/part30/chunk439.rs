//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 439/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk439(t295: f64, t312: f64, t7091: f64, t1248: f64, t6353: f64, t296: f64, t1091: f64, t6360: f64, t2881: f64, t1212: f64, t1501: f64) -> (f64, f64, f64, f64, f64) {
    let t7093 = t295 * t7091 * t312;
    let t7097 = t6353 * t1248;
    let t7098 = t296 * t7097;
    let t7101 = t6360 * t1091;
    let t7102 = t2881 * t7101;
    let t7105 = t1501 * t1212;
    (t7093, t7098, t7101, t7102, t7105)
}
