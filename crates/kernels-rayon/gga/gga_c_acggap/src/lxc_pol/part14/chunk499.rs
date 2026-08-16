//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 499/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk499(t257: f64, t2754: f64, t249: f64, t743: f64, t62: f64, t70: f64, t746: f64, t2742: f64, t67: f64, t747: f64, t685: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2755 = t2754 * t257;
    let t2759 = 1.0_f64 / t743 / t249;
    let t2760 = t62 * t2759;
    let t2762 = 1.0_f64 / t746 / t70;
    let t2763 = t2742 * t2762;
    let t2767 = 1.0_f64 / t743 / t67;
    let t2768 = t62 * t2767;
    let t2769 = t2742 * t747;
    let t2773 = 1.0_f64 / t685 / t80;
    (t2755, t2760, t2763, t2768, t2769, t2773)
}
