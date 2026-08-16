//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 912/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk912(t18048: f64, t18083: f64, t202: f64, t4985: f64, t237: f64, t458: f64, t4966: f64, t17749: f64, t9568: f64, t92: f64, t17766: f64, t2404: f64) -> (f64, f64, f64, f64, f64) {
    let t18084 = t18048 + t18083;
    let t18089 = t202 * t4985;
    let t18090 = t18089 * t237;
    let t18096 = t458 * t4966;
    let t18098 = t9568 * t17749;
    let t18099 = t92 * t18098;
    let t18101 = t2404 * t17766;
    (t18084, t18090, t18096, t18099, t18101)
}
