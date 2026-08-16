//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 895/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk895(t226: f64, t2383: f64, t17817: f64, t3725: f64, t6: f64, t4952: f64, t2393: f64, t4947: f64, t3771: f64, t1109: f64, t4951: f64, t688: f64) -> (f64, f64, f64, f64) {
    let t17818 = t2383 * t226;
    let t17819 = t17817 * t17818;
    let t17820 = t3725 * t6;
    let t17821 = t17820 * t4952;
    let t17824 = t4947 * t2393;
    let t17825 = t3771 * t17824;
    let t17827 = t4951 * t1109 * t688;
    (t17819, t17821, t17825, t17827)
}
