//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 353/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk353(t30: f64, t33: f64, t1173: f64, t498: f64, t490: f64, t580: f64, t1006: f64, t493: f64, t162: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1175 = 4.0_f64 * t1173 * t498;
    let t1178 = piecewise3(t31, 0.0_f64, 4.0_f64 / 3.0_f64 * t490 * t580);
    let t1181 = piecewise3(t34, 0.0_f64, 4.0_f64 / 3.0_f64 * t493 * t1006);
    let t1183 = (t1178 + t1181) * t162;
    (t1175, t1183)
}
