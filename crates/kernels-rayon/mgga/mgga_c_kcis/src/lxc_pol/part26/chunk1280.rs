//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1280/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1280(t1394: f64, t28499: f64, t5655: f64, t4153: f64, t5663: f64, t18210: f64, t29513: f64, t7978: f64, t1464: f64, t2011: f64, t27387: f64, t52073: f64) -> (f64, f64, f64, f64, f64) {
    let t101938 = t1394 * t28499 * t5655;
    let t101941 = t4153 * t28499 * t5663;
    let t101943 = t18210 * t29513;
    let t101944 = t7978 * t101943;
    let t101948 = t1464 * t27387 * t52073 * t2011;
    (t101938, t101941, t101943, t101944, t101948)
}
