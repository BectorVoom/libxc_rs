//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 434/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk434(t3103: f64, t370: f64, t27: f64, t89: f64, t1545: f64, t1548: f64, t1551: f64, t2981: f64, t2986: f64, t2990: f64, t2995: f64, t3003: f64, t3006: f64, t3011: f64, t3016: f64) -> (f64, f64, f64) {
    let t3104 = t370 * t3103;
    let t3106 = t89 * t27 * t3104;
    let t3108 = t1545 + t1548 / 54.0_f64 + t1551 / 18.0_f64 + t2981 / 54.0_f64 - t2986 / 27.0_f64 + t2990 / 18.0_f64 + t2995 / 9.0_f64 - t3003 / 9.0_f64 + t3006 / 18.0_f64 + t3011 / 18.0_f64 + t3016 / 3.0_f64 - t3106 / 6.0_f64;
    (t3104, t3106, t3108)
}
