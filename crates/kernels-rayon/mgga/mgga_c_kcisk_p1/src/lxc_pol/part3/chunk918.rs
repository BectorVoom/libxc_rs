//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 918/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk918(t1273: f64, t4101: f64, t373: f64, t4128: f64, t13561: f64, t4100: f64, t1275: f64, t4120: f64, t1233: f64, t4023: f64, t4126: f64, t1229: f64, t4080: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13562 = t4101 * t1273;
    let t13565 = 1.0_f64 / t4128 / t373;
    let t13566 = t13561 * t13562 * t13565;
    let t13569 = t4100 * t1273;
    let t13570 = t1275 * t4120;
    let t13571 = t13569 * t13570;
    let t13574 = t4023 * t1233;
    let t13578 = t4126 * t13562 * t1275;
    let t13583 = t1229 * t4080;
    (t13562, t13566, t13571, t13574, t13578, t13583)
}
