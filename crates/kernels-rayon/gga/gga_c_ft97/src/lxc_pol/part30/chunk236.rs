//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 236/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk236(t2334: f64, t681: f64, t756: f64, t89: f64, t2399: f64, t259: f64, t1882: f64, t731: f64, t768: f64, t257: f64, t760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2518 = 4.0_f64 / 9.0_f64 * t2334;
    let t2533 = 4.0_f64 / 27.0_f64 * t2334;
    let t2549 = t89 * t681 * t756;
    let t2553 = 4.0_f64 / 27.0_f64 * t89 * t2399 * t259;
    let t2554 = t1882 * t731;
    let t2556 = t1882 * t768;
    let t2567 = 1.0_f64 / t760 / t257;
    (t2518, t2533, t2549, t2553, t2554, t2556, t2567)
}
