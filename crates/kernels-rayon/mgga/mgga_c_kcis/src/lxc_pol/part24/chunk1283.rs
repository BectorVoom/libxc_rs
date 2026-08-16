//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1283/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1283(t2811: f64, t6539: f64, t1008: f64, t26686: f64, t13376: f64, t1662: f64, t4947: f64, t14554: f64, t4621: f64, t4781: f64, t27819: f64, t6276: f64) -> (f64, f64, f64, f64) {
    let t101001 = t2811 * t6539;
    let t101003 = t26686 * t101001 * t1008;
    let t101012 = t4947 * t13376 * t1662;
    let t101018 = t14554 * t4781 * t4621;
    let t101028 = t4947 * t27819 * t6276 * t1008;
    (t101003, t101012, t101018, t101028)
}
