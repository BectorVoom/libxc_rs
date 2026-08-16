//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 524/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk524(t1786: f64, t971: f64, t463: f64, t3539: f64, t604: f64, t135: f64, t3347: f64, t131: f64, t538: f64, t71: f64, t929: f64, t1045: f64, t2178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11902 = t1786 * t971;
    let t11906 = t463 * t971;
    let t12277 = t3539 * t604;
    let t12374 = t3347 * t135;
    let t12411 = t538 * t131;
    let t12486 = t71 * t929;
    let t12664 = t1045 * t2178;
    (t11902, t11906, t12277, t12374, t12411, t12486, t12664)
}
