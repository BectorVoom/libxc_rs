//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1240/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1240(t1018: f64, t1079: f64, t11082: f64, t11920: f64, t11924: f64, t11926: f64, t1305: f64, t1306: f64, t1307: f64, t1308: f64, t2405: f64, t330: f64, t3381: f64, t3643: f64, t3645: f64, t40767: f64, t40869: f64, t837: f64, t838: f64, t8420: f64) -> f64 {
    let t40892 = (t40767 + t40869) * t330 + 2.0_f64 * t11920 * t837 * t330 + t3643 * t1305 * t330 + t3643 * t1307 * t330 + t11082 * t1018 * t330 + 2.0_f64 * t3381 * t2405 * t330 + 2.0_f64 * t11924 * t838 + t1079 * t8420 * t330 + 2.0_f64 * t11926 * t838 + t3645 * t1306 + t3645 * t1308;
    t40892
}
