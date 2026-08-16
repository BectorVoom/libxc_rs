//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 634/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk634(t332: f64, t5429: f64, t2917: f64, t2918: f64, t4917: f64, t1091: f64, t1268: f64, t2923: f64, t231: f64, t2928: f64, t4635: f64, t893: f64) -> (f64, f64, f64, f64, f64) {
    let t5430 = t5429 * t332;
    let t5442 = t2917 * t2918 * t4917;
    let t5446 = t2923 * t1091 * t1268;
    let t5450 = t231 * t2928 * t4917;
    let t5454 = t231 * t893 * t4635;
    (t5430, t5442, t5446, t5450, t5454)
}
