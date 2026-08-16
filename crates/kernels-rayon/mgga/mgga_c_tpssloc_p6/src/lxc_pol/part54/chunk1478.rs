//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1478/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1478(t120800: f64, t120803: f64, t122776: f64, t122780: f64, t122784: f64, t122786: f64, t122788: f64, t122790: f64, t122794: f64, t122800: f64, t125024: f64, t33195: f64, t577: f64, t7956: f64, t85416: f64) -> f64 {
    let t125029 = t33195 + t122776 + t122780 + 0.45e1_f64 * t125024 * t577 + 27.0_f64 * t85416 * t7956 + t122784 + t122786 + t122788 + t122790 + t120800 + t120803 + t122794 + t122800;
    t125029
}
