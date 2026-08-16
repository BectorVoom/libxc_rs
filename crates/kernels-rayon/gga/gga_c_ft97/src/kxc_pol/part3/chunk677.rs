//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 677/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk677(t8282: f64, t849: f64, t303: f64, t3051: f64, t1771: f64, t854: f64, t10491: f64, t2: f64, t10478: f64, t305: f64, t631: f64, t7242: f64, t798: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10586 = t8282 * t849;
    let t10594 = 28.0_f64 / 27.0_f64 * t3051 * t303;
    let t10595 = t1771 * t854;
    let t10603 = t10491 * t2;
    let t10613 = t10478 * t2;
    let t10631 = 1.0_f64 / t305 / t631 / t898 / t798 / t7242 / 4.0_f64;
    (t10586, t10594, t10595, t10603, t10613, t10631)
}
