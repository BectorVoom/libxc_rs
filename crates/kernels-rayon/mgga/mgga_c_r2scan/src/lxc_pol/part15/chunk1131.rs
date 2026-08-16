//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1131/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1131(t10879: f64, t11741: f64, t37660: f64, t39540: f64, t39542: f64, t39545: f64, t39549: f64, t39550: f64, t39552: f64, t39554: f64, t39558: f64, t39561: f64, t39563: f64) -> f64 {
    let t39565 = t10879 * t11741;
    let t39567 = -0.43663693315433241792e-2_f64 * t39540 - 0.16463622957338778997e0_f64 * t39542 - 0.2600466522016280569e0_f64 * t39545 - 0.14282990759302185292e-1_f64 * t37660 - t39549 - 0.54878743191129263322e-1_f64 * t39550 - 0.43341108700271342816e-1_f64 * t39552 - 0.86682217400542685632e-1_f64 * t39554 - 0.22511059664845582436e0_f64 * t39558 - 0.43341108700271342816e-1_f64 * t39561 - 0.13002332610081402845e0_f64 * t39563 - 0.2600466522016280569e0_f64 * t39565;
    t39567
}
