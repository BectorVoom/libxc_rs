//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 780/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk780(t2766: f64, t9921: f64, t4199: f64, t9583: f64, t10422: f64, t2771: f64, t10426: f64, t2: f64, t7640: f64, t10262: f64, t192: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10560 = t2766 * t9921;
    let t10563 = t4199 * t9583;
    let t10566 = t2771 * t10422;
    let t10568 = t2771 * t10426;
    let t10570 = t7640 * t2;
    let t10572 = t192 * t10570 * t10262;
    let t10575 = t2 * t824;
    (t10560, t10563, t10566, t10568, t10572, t10575)
}
