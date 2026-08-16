//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 987/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk987(t7799: f64, t8571: f64, t1980: f64, t3201: f64, t7458: f64, t8569: f64, t1988: f64, t8549: f64, t1095: f64, t4806: f64, t7476: f64, t8555: f64) -> (f64, f64, f64, f64, f64) {
    let t34771 = t7799 * t8571;
    let t34783 = t1980 * t7458 * t3201 * t8569;
    let t34794 = t1988 * t8549;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34804 = t7799 * t8555;
    (t34771, t34783, t34794, t34802, t34804)
}
