//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 941/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk941(t31867: f64, t2138: f64, t2147: f64, t463: f64, t8064: f64, t2132: f64, t3037: f64, t32146: f64, t633: f64, t7885: f64, t8336: f64, t2219: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t32967 = 0.2767432121485165382e-1_f64 * t31867;
    let t32990 = t2138 * t2147 * t8064 * t463;
    let t32997 = 0.10408353825846239354e2_f64 * t32146 * t2132 * t633 * t3037;
    let t33000 = t7885 * t2147 * t8336 * t463;
    let t33008 = t848 * t2219;
    (t32967, t32990, t32997, t33000, t33008)
}
