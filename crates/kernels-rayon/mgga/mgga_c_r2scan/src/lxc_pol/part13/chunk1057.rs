//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1057/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1057(t10648: f64, t10649: f64, t1375: f64, t58: f64, t597: f64, t10650: f64, t1654: f64, t10673: f64, t10674: f64, t10676: f64, t874: f64, t10680: f64, t10682: f64) -> (f64, f64, f64, f64, f64) {
    let t37488 = t10648 * t10649 * t58 * t1375 * t597;
    let t37495 = t10648 * t10649 * t10650 * t1654;
    let t37499 = t10673 * t10674 * t1375 * t10676;
    let t37501 = t1654 * t874;
    let t37503 = t10680 * t10682 * t37501;
    (t37488, t37495, t37499, t37501, t37503)
}
