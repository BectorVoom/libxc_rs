//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 914/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk914(t22735: f64, t7837: f64, t22833: f64, t66: f64, t5532: f64, t77: f64, t1602: f64, t409: f64, t39: f64, t5585: f64) -> (f64, f64, f64, f64, f64) {
    let t92314 = t7837 * t22735;
    let t92335 = t22833 * t66;
    let t92339 = t77 * t5532;
    let t92353 = t1602 * t409;
    let t92354 = t39 * t5585;
    (t92314, t92335, t92339, t92353, t92354)
}
