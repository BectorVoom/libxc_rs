//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 318/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk318(t1053: f64, t1102: f64, t1104: f64, t1051: f64, t1056: f64, t1062: f64) -> (f64, f64) {
    let t1106 = t1102 * t1053 * t1104;
    let t1114 = 0.54878743191129263322e-1_f64 * t1051 + 0.86682217400542685632e-1_f64 * t1056 - 0.43663693315433241794e-2_f64 * t1062;
    (t1106, t1114)
}
