//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 439/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk439(t27: f64, t6681: f64, t89: f64, t5898: f64, t5915: f64, t6659: f64, t6663: f64, t6667: f64, t6671: f64, t6675: f64, t6679: f64) -> (f64, f64) {
    let t6683 = t89 * t27 * t6681;
    let t6685 = t6659 / 12.0_f64 + t5898 + t6663 / 18.0_f64 + t6667 / 3.0_f64 - t6671 / 6.0_f64 + t5915 + t6675 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t6679 - t6683 / 3.0_f64;
    (t6683, t6685)
}
