//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 311/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk311(t1267: f64, t1268: f64, t1015: f64, t1025: f64, t1089: f64, t1124: f64, t1128: f64, t1136: f64, t1236: f64, t1240: f64, t430: f64, t436: f64) -> (f64, f64, f64, f64) {
    let t1269 = t1267 * t1268;
    let t1272 = 0.11607361111111111111e-2_f64 * t1015;
    let t1278 = t1236 * t430 - 0.66725e-1_f64 * t1240 * t1269 + t1272 + 0.11607361111111111111e-2_f64 * t1025 + 0.17411041666666666666e-2_f64 * t1089 - 0.17411041666666666666e-2_f64 * t1124 - 0.46429444444444444443e-2_f64 * t1128 + 0.11607361111111111111e-2_f64 * t1136;
    let t1280 = t436 * t436;
    (t1269, t1272, t1278, t1280)
}
