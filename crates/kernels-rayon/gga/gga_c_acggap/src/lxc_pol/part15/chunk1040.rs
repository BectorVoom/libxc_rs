//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1040/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1040(t1998: f64, t5251: f64, t1967: f64, t8566: f64, t4557: f64, t309: f64, t556: f64, t322: f64, t406: f64, t944: f64, t1539: f64, t463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36388 = t1998 * t5251;
    let t36390 = t1967 * t8566;
    let t36392 = t1998 * t4557;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36429 = t944 * t309 * t406;
    let t36475 = t1539 * t309;
    let t36479 = t1539 * t463;
    (t36388, t36390, t36392, t36417, t36429, t36475, t36479)
}
