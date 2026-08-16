//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 436/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk436(t1780: f64, t2: f64, t2984: f64, t1787: f64, t2988: f64, t463: f64, t2993: f64, t17: f64, t3050: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3127 = t1780 * t2;
    let t3128 = t3127 * t2984;
    let t3131 = t1787 * t2988;
    let t3134 = t463 * t2;
    let t3135 = t3134 * t2993;
    let t3139 = t9 * t3050 * t17;
    (t3127, t3128, t3131, t3134, t3135, t3139)
}
