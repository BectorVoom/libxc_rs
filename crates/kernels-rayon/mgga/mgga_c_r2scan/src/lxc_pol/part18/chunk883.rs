//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 883/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk883(t2719: f64, t938: f64, t551: f64, t552: f64, t2207: f64, t2691: f64, t2837: f64, t3016: f64, t788: f64, t785: f64, t113: f64, t8837: f64) -> (f64, f64, f64, f64, f64) {
    let t9407 = t938 * t2719;
    let t9409 = t551 * t552 * t9407;
    let t9416 = t2207 * t2837 * t2691;
    let t9418 = t788 * t3016;
    let t9420 = t2207 * t785 * t9418;
    let t9422 = t8837 * t113;
    (t9409, t9416, t9418, t9420, t9422)
}
