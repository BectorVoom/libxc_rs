//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 920/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk920(t1131: f64, t3972: f64, t729: f64, t762: f64, t4934: f64, t713: f64, t10157: f64, t265: f64, t5064: f64, t2568: f64, t766: f64, t10052: f64) -> (f64, f64, f64, f64) {
    let t18201 = t1131 * t3972;
    let t18203 = t729 * t762 * t18201;
    let t18206 = t4934 * t713;
    let t18208 = t10157 * t265 * t18206;
    let t18211 = t5064 * t713;
    let t18213 = t729 * t2568 * t18211;
    let t18216 = t5064 * t766;
    let t18217 = t10052 * t18216;
    (t18203, t18208, t18213, t18217)
}
