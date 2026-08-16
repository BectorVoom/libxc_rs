//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 553/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk553(t1594: f64, t4441: f64, t930: f64, t938: f64, t374: f64, t35: f64) -> (f64, f64, f64, f64) {
    let t4442 = t1594 * t4441;
    let t4445 = t930 * t938;
    let t4446 = t374 * t4445;
    let t4449 = t4441 * t35;
    (t4442, t4445, t4446, t4449)
}
