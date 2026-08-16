//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 676/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk676(t10478: f64, t309: f64, t2347: f64, t870: f64, t2680: f64, t665: f64, t2360: f64, t2399: f64, t865: f64, t89: f64, t10400: f64, t295: f64, t9567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10479 = t10478 * t309;
    let t10485 = t870 * t2347;
    let t10491 = t665 * t2680;
    let t10492 = t10491 * t309;
    let t10503 = t870 * t2360;
    let t10514 = t89 * t2399 * t865;
    let t10553 = 4.0_f64 / 9.0_f64 * t10400;
    let t10580 = t9567 * t295;
    (t10479, t10485, t10491, t10492, t10503, t10514, t10553, t10580)
}
