//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1177/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1177(t29274: f64, t3332: f64, t7614: f64, t1060: f64, t269: f64, t783: f64, t9083: f64, t12550: f64, t788: f64, t37616: f64, t37630: f64, t37634: f64, t37639: f64, t39500: f64, t39503: f64, t39512: f64, t39523: f64, t41405: f64) -> f64 {
    let t43072 = t7614 * t3332 * t29274;
    let t43076 = t783 * t9083 * t269 * t1060;
    let t43079 = t783 * t12550 * t788;
    let t43081 = t41405 - 0.42377972951376424087e0_f64 * t37616 - 0.59512461497092438715e-1_f64 * t37630 - 0.17853738449127731614e0_f64 * t37634 - 0.14457274399185490173e-3_f64 * t37639 - 0.26198215989259945075e-1_f64 * t43072 + t39500 - t39503 + t39512 - 0.21831846657716620896e-2_f64 * t43076 + 0.23287303101564395623e-1_f64 * t43079 + t39523;
    t43081
}
