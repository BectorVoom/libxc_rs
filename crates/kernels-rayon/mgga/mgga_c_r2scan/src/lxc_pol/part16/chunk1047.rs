//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1047/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1047(t3416: f64, t6755: f64, t1096: f64, t19309: f64, t113: f64, t3268: f64, t97: f64, t10666: f64, t3347: f64, t5086: f64, t1064: f64, t23040: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37223 = t6755 * t3416;
    let t37226 = t19309 * t1096;
    let t37271 = t97 * t3268 * t113;
    let t37282 = t97 * t10666 * t113;
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    (t37223, t37226, t37271, t37282, t37292, t37299)
}
