//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 684/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk684(t30: f64, t33: f64, t187: f64, t4377: f64, t1288: f64, t3217: f64, t1197: f64, t2: f64, t555: f64, t580: f64, t1497: f64, t3225: f64, t1201: f64, t1006: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t4379 = 0.19751673498613801407e-1_f64 * t4377 * t187;
    let t4380 = t3217 * t1288;
    let t4383 = t1197 * t2;
    let t4387 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t4380 * t580 + 4.0_f64 / 3.0_f64 * t4383 * t555);
    let t4388 = t3225 * t1497;
    let t4391 = t1201 * t2;
    let t4395 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t4388 * t1006 - 4.0_f64 / 3.0_f64 * t4391 * t555);
    (t4379, t4380, t4387, t4388, t4395)
}
