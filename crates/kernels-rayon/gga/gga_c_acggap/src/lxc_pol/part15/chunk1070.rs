//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1070/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1070(t2131: f64, t2147: f64, t309: f64, t9417: f64, t463: f64, t9431: f64, t2132: f64, t2138: f64, t322: f64, t9367: f64, t8073: f64, t8397: f64) -> (f64, f64, f64, f64) {
    let t38153 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t9417 * t309;
    let t38157 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t9431 * t463;
    let t38165 = 0.17347256376410398924e1_f64 * t2138 * t2132 * t9367 * t322;
    let t38176 = 0.34694512752820797848e1_f64 * t8397 * t8073;
    (t38153, t38157, t38165, t38176)
}
