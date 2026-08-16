//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 628/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk628(t28128: f64, t3864: f64, t14127: f64, t14159: f64, t6075: f64, t24737: f64, t3859: f64, t13885: f64, t255: f64, t9707: f64, t3837: f64, t6074: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28129 = t28128 * t3864;
    let t28130 = t14127 * t28129;
    let t28133 = t14159 * t6075;
    let t28136 = t24737 * t3859;
    let t28137 = t13885 * t28136;
    let t28140 = t9707 * t255;
    let t28141 = t6074 * t3837;
    (t28129, t28130, t28133, t28136, t28137, t28140, t28141)
}
