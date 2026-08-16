//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 667/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk667(t693: f64, t10: f64, t242: f64, t3050: f64, t1636: f64, t714: f64, t89: f64, t191: f64, t7514: f64, t2344: f64, t375: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9680 = t693 * t693;
    let t9681 = 1.0_f64 / t9680;
    let t9698 = t10 * t3050 * t242;
    let t9699 = 14.0_f64 / 81.0_f64 * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9707 = t191 * t7514;
    let t9725 = t375 * t2344;
    let t9733 = t1636 * t665;
    (t9681, t9698, t9699, t9701, t9707, t9725, t9733)
}
