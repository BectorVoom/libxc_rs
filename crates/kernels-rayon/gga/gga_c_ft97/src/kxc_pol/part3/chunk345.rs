//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 345/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk345(t10: f64, t144: f64, t1542: f64, t1546: f64, t520: f64, t89: f64, t375: f64, t559: f64, t143: f64, t1557: f64, t378: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1956 = t10 * t1542 * t144;
    let t1957 = 2.0_f64 / 27.0_f64 * t1956;
    let t1959 = t89 * t1546 * t520;
    let t1960 = t1959 / 27.0_f64;
    let t1962 = t89 * t375 * t559;
    let t1963 = t1962 / 9.0_f64;
    let t1964 = t143 * t1557;
    let t1969 = t378 * t525;
    (t1956, t1957, t1959, t1960, t1962, t1963, t1964, t1969)
}
