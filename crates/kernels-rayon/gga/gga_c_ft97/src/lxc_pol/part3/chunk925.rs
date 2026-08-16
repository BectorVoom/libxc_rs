//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 925/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk925(t1775: f64, t5099: f64, t5106: f64, t16579: f64, t738: f64, t737: f64, t18139: f64, t192: f64, t743: f64, t458: f64, t5118: f64, t5114: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18303 = t1775 * t5099;
    let t18305 = t1775 * t5106;
    let t18307 = t738 * t16579;
    let t18308 = t737 * t18307;
    let t18312 = t192 * t743 * t18139;
    let t18314 = t458 * t5118;
    let t18316 = t458 * t5114;
    (t18303, t18305, t18307, t18308, t18312, t18314, t18316)
}
