//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 798/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk798(t219: f64, t4434: f64, t4443: f64, t516: f64, t73: f64, t1246: f64, t1625: f64, t1206: f64, t1228: f64, t4397: f64, t1226: f64, t1229: f64, t1634: f64, t1636: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4445 = (t4434 + t4443) * t219;
    let t4451 = t516 * t73;
    let t4452 = t1246 * t1625;
    let t4453 = t4452 * t1206;
    let t4456 = t1228 * t4397;
    let t4459 = 3.0_f64 * t1226 * t1636 + 3.0_f64 * t1229 * t1634 - t4445 * t518 - 12.0_f64 * t4451 * t4453 + 3.0_f64 * t4456 * t516;
    (t4445, t4451, t4452, t4453, t4456, t4459)
}
