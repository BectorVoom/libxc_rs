//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 425/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk425(t1642: f64, t2984: f64, t92: f64, t2993: f64, t378: f64, t12: f64, t2998: f64) -> (f64, f64, f64, f64, f64) {
    let t3044 = t1642 * t2984;
    let t3045 = t92 * t3044;
    let t3047 = t378 * t2993;
    let t3048 = t92 * t3047;
    let t3050 = t12 * t2998;
    (t3044, t3045, t3047, t3048, t3050)
}
