//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1222/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1222(t3263: f64, t3275: f64, t40667: f64, t3446: f64, t37475: f64, t970: f64, t1065: f64, t2526: f64, t3270: f64, t10667: f64, t105: f64, t2530: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t40670 = 3.0_f64 / 2.0_f64 * t3275 * t3263 * t40667;
    let t40672 = t3446 * t37475 * t970;
    let t40676 = t1065 * t2526;
    let t40677 = t3270 * t40676;
    let t40679 = 3.0_f64 / 2.0_f64 * t10667 * t40677;
    let t40681 = t97 * t105 * t2530;
    (t40670, t40672, t40679, t40681)
}
