//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1232/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1232(t11894: f64, t833: f64, t1013: f64, t1074: f64, t11060: f64, t11066: f64, t1292: f64, t1295: f64, t1300: f64, t2394: f64, t2400: f64, t3370: f64, t3633: f64, t37020: f64, t37023: f64, t6693: f64, t829: f64, t8398: f64, t8409: f64, t8412: f64, t8415: f64) -> f64 {
    let t40764 = t11894 * t833;
    let t40767 = -0.768e1_f64 * t37020 * t2400 - 0.768e1_f64 * t11066 * t8412 - 0.384e1_f64 * t11066 * t8415 - 0.1536e2_f64 * t37023 * t8409 - 0.128e1_f64 * t1300 * t11060 * t1013 - 0.256e1_f64 * t1300 * t3370 * t2394 - 0.128e1_f64 * t1300 * t1074 * t8398 - 0.256e1_f64 * t1300 * t11894 * t829 - 0.128e1_f64 * t1300 * t3633 * t1292 - 0.384e1_f64 * t6693 * t3633 * t1295 - 0.256e1_f64 * t40764 * t829;
    t40767
}
