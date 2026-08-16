//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1099/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1099(t1349: f64, t34966: f64, t376: f64, t32706: f64, t6580: f64, t32699: f64, t35007: f64, t5769: f64, t138480: f64, t138493: f64, t138521: f64, t138524: f64, t26533: f64, t26538: f64, t28: f64, t32709: f64, t35206: f64, t39653: f64, t5849: f64, t609: f64) -> f64 {
    let t147142 = t1349 * t376 * t34966;
    let t147144 = t6580 * t32706;
    let t147152 = t6580 * t32699;
    let t147154 = t35007 * t5769;
    let t147159 = -t138480 + t138493 / 27.0_f64 + t35007 * t5849 / 6.0_f64 - t1349 * t28 * t32709 * t26538 / 3.0_f64 - t147142 / 18.0_f64 - t147144 / 18.0_f64 - t1349 * t28 * t32709 * t26533 / 3.0_f64 - t138521 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t138524 - t147152 / 9.0_f64 - t147154 / 18.0_f64 + 48.0_f64 * t39653 * t35206 * t609;
    t147159
}
