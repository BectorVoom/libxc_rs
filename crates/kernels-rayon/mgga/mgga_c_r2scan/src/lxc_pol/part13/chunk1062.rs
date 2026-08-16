//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1062/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1062(t37599: f64, t2150: f64, t37470: f64, t574: f64, t10810: f64, t6402: f64, t2101: f64, t547: f64, t2096: f64, t265: f64, t267: f64, t546: f64, t6476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37600 = 0.21476142888649427853e-4_f64 * t37599;
    let t37616 = t574 * t37470 * t2150;
    let t37619 = t574 * t10810 * t6402;
    let t37625 = t547 * t2101;
    let t37628 = t2096 * t265 * t267;
    let t37630 = t546 * t37625 * t37628 * t6476;
    (t37600, t37616, t37619, t37625, t37628, t37630)
}
