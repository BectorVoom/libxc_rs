//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 908/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk908(t1068: f64, t2973: f64, t2998: f64, t425: f64, t9347: f64, t9172: f64, t9213: f64, t1049: f64, t2954: f64, t2953: f64, t417: f64, t412: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9359 = t1068 * t2973;
    let t9370 = t1068 * t2998;
    let t9373 = t425 * t9347;
    let t9380 = t425 * t9172;
    let t9399 = 0.55403703703703703703e-1_f64 * t9213;
    let t9419 = t1049 * t2954;
    let t9423 = 1.0_f64 / t2953 / t417;
    let t9424 = t412 * t9423;
    (t9359, t9370, t9373, t9380, t9399, t9419, t9424)
}
