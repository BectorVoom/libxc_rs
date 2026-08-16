//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 794/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk794(t378: f64, t538: f64, t39: f64, t8907: f64, t40: f64, t3392: f64) -> (f64, f64, f64, f64) {
    let t32768 = t378 * t538;
    let t32772 = t8907 * t39;
    let t32773 = t32772 * t40;
    let t32774 = t3392 * t32773;
    (t32768, t32772, t32773, t32774)
}
