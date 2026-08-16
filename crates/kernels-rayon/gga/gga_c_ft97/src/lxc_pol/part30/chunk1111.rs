//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1111/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1111(t1091: f64, t143144: f64, t2665: f64, t6317: f64, t152661: f64, t24976: f64, t24980: f64, t1212: f64, t143293: f64, t193: f64, t89: f64, t152772: f64) -> (f64, f64, f64, f64) {
    let t152917 = t6317 * t2665 * t143144 * t1091;
    let t152920 = t24980 * t24976 * t152661;
    let t152924 = t89 * t193 * t143293 * t1212;
    let t152927 = t6317 * t24976 * t152772;
    (t152917, t152920, t152924, t152927)
}
