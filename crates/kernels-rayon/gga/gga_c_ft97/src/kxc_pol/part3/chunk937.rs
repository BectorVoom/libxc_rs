//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 937/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk937(t18486: f64, t729: f64, t762: f64, t5147: f64, t766: f64, t2568: f64, t242: f64, t18: f64, t992: f64, t3885: f64, t2606: f64, t18459: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18488 = t729 * t762 * t18486;
    let t18491 = t5147 * t766;
    let t18492 = t2568 * t18491;
    let t18493 = t242 * t18492;
    let t18497 = t992 * t18;
    let t18498 = t3885 * t18497;
    let t18499 = t2606 * t18498;
    let t18502 = t3885 * t18459;
    (t18488, t18492, t18493, t18497, t18499, t18502)
}
