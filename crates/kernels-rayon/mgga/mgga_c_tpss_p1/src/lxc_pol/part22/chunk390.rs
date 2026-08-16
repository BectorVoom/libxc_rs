//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 390/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk390(t1163: f64, t1168: f64, t118: f64, t1273: f64, t485: f64, t488: f64, t544: f64, t624: f64, t626: f64, t646: f64, t3: f64, t546: f64) -> (f64, f64, f64) {
    let t1275 = -t1163 * t118 + t1168 * t544 + t1273 * t488 - t485 * t624 - 2.0_f64 * t626 * t646;
    let t1276 = t3 * t1275;
    let t1278 = t3 * t546;
    (t1275, t1276, t1278)
}
