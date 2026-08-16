//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 391/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk391(t505: f64, t6334: f64, t2665: f64, t446: f64, t6222: f64, t824: f64, t193: f64, t89: f64, t6260: f64, t799: f64, t27: f64, t6312: f64, t6316: f64, t6321: f64, t6325: f64, t6329: f64, t6333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6335 = t6334 * t505;
    let t6336 = t2665 * t6335;
    let t6337 = t446 * t6336;
    let t6339 = t6222 * t824;
    let t6340 = t193 * t6339;
    let t6341 = t89 * t6340;
    let t6343 = t799 * t6260;
    let t6345 = t89 * t27 * t6343;
    let t6347 = t6312 / 12.0_f64 + t6316 + t6321 / 18.0_f64 + t6325 / 3.0_f64 - t6329 / 6.0_f64 + t6333 + t6337 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t6341 - t6345 / 3.0_f64;
    (t6335, t6336, t6337, t6339, t6340, t6341, t6343, t6345, t6347)
}
