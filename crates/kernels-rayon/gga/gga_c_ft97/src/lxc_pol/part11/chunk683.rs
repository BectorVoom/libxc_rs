//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 683/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk683(t147: f64, t9478: f64, t2252: f64, t342: f64, t657: f64, t173: f64, t703: f64) -> (f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t9479 = piecewise3(t148, 0.0_f64, t9478);
    let t9482 = t342 * t2252 * t657 / 18.0_f64;
    let t9483 = t173 * t703;
    (t9479, t9482, t9483)
}
