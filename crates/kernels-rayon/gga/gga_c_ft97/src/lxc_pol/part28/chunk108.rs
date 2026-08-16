//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 108/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk108(t14: f64, t391: f64, t72: f64, t68: f64, t172: f64, t47: f64, t67: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t410 = t391 * t14;
    let t411 = t410 * t72;
    let t412 = t68 * t411;
    let t414 = t47 * t172;
    let t415 = t414 * t72;
    let t416 = t68 * t415;
    let t417 = 0.6384360837962962963e-2_f64 * t416;
    let t419 = t9 * t67 * t47;
    (t410, t412, t414, t416, t417, t419)
}
