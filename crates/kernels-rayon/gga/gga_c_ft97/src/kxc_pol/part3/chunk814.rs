//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 814/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk814(t1022: f64, t16658: f64, t3413: f64, t4649: f64, t1952: f64, t4719: f64, t3450: f64, t925: f64, t9073: f64, t446: f64, t1017: f64, t363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16659 = t16658 * t1022;
    let t16661 = t4649 * t3413;
    let t16664 = t1952 * t4719;
    let t16666 = t925 * t3450;
    let t16667 = t9073 * t16666;
    let t16668 = t446 * t16667;
    let t16670 = t1017 * t363;
    (t16659, t16661, t16664, t16666, t16668, t16670)
}
