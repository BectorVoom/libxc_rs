//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 777/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk777(t30: f64, t33: f64, t1197: f64, t3217: f64, t4578: f64, t5328: f64, t1201: f64, t3225: f64, t5059: f64, t5335: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5358 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t3217 * t5328 + 2.0_f64 / 3.0_f64 * t1197 * t4578);
    let t5364 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t3225 * t5335 + 2.0_f64 / 3.0_f64 * t1201 * t5059);
    let t5366 = t5358 / 2.0_f64 + t5364 / 2.0_f64;
    t5366
}
