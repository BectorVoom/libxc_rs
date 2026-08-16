//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 752/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk752(t6188: f64, t6461: f64, t6072: f64, t2168: f64, t2183: f64, t2097: f64, t547: f64, t546: f64, t560: f64, t6212: f64, t6211: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6462 = t6188 * t6461;
    let t6463 = t6462 * t6072;
    let t6465 = t2183 * t2168;
    let t6474 = t547 * t2097;
    let t6475 = t546 * t6474;
    let t6476 = t6212 * t560;
    let t6477 = t6211 * t6476;
    let t6478 = t6475 * t6477;
    let t6480 = t565 * t6474;
    (t6462, t6463, t6465, t6475, t6476, t6478, t6480)
}
