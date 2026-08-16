//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 400/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk400(t287: f64, t800: f64, t194: f64, t272: f64, t123: f64, t805: f64, t289: f64, t815: f64) -> (f64, f64, f64, f64) {
    let t2691 = t800 * t287;
    let t2697 = 1.0_f64 / t272 / t194;
    let t2710 = t123 / t805 / t194;
    let t2724 = 1.0_f64 / t815 / t289;
    (t2691, t2697, t2710, t2724)
}
