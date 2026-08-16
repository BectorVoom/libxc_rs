//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 784/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk784(t1151: f64, t1153: f64, t198: f64, t330: f64, t4023: f64, t4062: f64, t4065: f64, t4067: f64, t4070: f64, t4107: f64, t4111: f64, t4189: f64, t4191: f64, t4194: f64, t4196: f64, t4200: f64, t4204: f64, t4209: f64, t4325: f64, t4329: f64) -> f64 {
    let t4332 = t1153 * t198 * t330 * t4325 - t1151 * t4023 * t4329 - t4062 + t4065 + t4067 - t4070 + t4107 + t4111 + t4189 + t4191 - t4194 - t4196 + t4200 - t4204 - t4209;
    t4332
}
