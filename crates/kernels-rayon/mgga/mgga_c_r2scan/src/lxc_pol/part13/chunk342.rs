//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 342/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk342(t1266: f64, t317: f64, t1250: f64, t1253: f64, t1258: f64, t1262: f64, t313: f64) -> (f64, f64) {
    let t1288 = 11.0_f64 / 9.0_f64 * t317 * t1266;
    let t1289 = 3.0_f64 / 10.0_f64 * t313 * (10.0_f64 / 9.0_f64 * t1250 + 5.0_f64 / 3.0_f64 * t1253 + 10.0_f64 / 9.0_f64 * t1258 + 5.0_f64 / 3.0_f64 * t1262) + t1288;
    (t1288, t1289)
}
