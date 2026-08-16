//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 585/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk585(t106: f64, t2333: f64, t3245: f64, t97: f64, t1418: f64, t1421: f64, t1424: f64, t1459: f64, t1463: f64, t1470: f64, t1480: f64, t1488: f64, t1511: f64, t1526: f64, t1529: f64, t1533: f64, t2872: f64, t3020: f64, t3036: f64, t3038: f64) -> f64 {
    let t3248 = t97 * t106 * t3245 * t2333;
    let t3249 = -t1418 - t1421 - t1424 - t1511 + t1459 - t1526 - 0.4726e1_f64 * t2872 + t3020 + t1470 - t1480 - t1488 - t3038 - t3036 - t1529 + t1463 - t1533 + t3248;
    t3249
}
