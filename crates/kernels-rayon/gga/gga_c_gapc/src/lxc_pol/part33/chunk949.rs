//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 949/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk949(t11589: f64, t6: f64, t8715: f64, t11588: f64, t11302: f64, t5395: f64, t5974: f64, t2994: f64, t435: f64) -> (f64, f64, f64, f64, f64) {
    let t11591 = t11589 * t6 * t8715;
    let t11592 = t11588 * t11591;
    let t11594 = t5395 * t11302;
    let t11595 = t11594 * t5974;
    let t11597 = t435 * t2994;
    (t11591, t11592, t11594, t11595, t11597)
}
