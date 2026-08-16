//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 764/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk764(t2469: f64, t7553: f64, t242: f64, t33490: f64, t1882: f64, t7555: f64, t2574: f64, t265: f64, t33346: f64, t7484: f64, t766: f64, t729: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33650 = t2469 * t7553;
    let t33651 = t242 * t33650;
    let t33654 = t242 * t33490;
    let t33658 = t1882 * t7555 / 9.0_f64;
    let t33660 = t2574 * t265 * t33346;
    let t33663 = t7484 * t766;
    let t33665 = t729 * t762 * t33663;
    (t33650, t33651, t33654, t33658, t33660, t33663, t33665)
}
