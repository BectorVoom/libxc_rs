//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 907/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk907(t6620: f64, t9415: f64, t3200: f64, t2822: f64, t6501: f64, t1662: f64, t4984: f64, t9517: f64, t1767: f64, t3217: f64, t4813: f64, t3202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19565 = t9415 * t6620;
    let t19566 = t3200 * t19565;
    let t19569 = t2822 * t6501;
    let t19571 = t1662 * t4984;
    let t19572 = t9517 * t19571;
    let t19573 = t3200 * t19572;
    let t19575 = t3217 * t1767;
    let t19576 = t19575 * t4813;
    let t19577 = t3202 * t19576;
    (t19566, t19569, t19571, t19573, t19576, t19577)
}
