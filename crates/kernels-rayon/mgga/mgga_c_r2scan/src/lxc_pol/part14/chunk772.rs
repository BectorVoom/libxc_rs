//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 772/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk772(t546: f64, t6474: f64, t560: f64, t6212: f64, t6211: f64, t565: f64, t481: f64, t133: f64, t2078: f64, t255: f64, t2168: f64, t2195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6475 = t546 * t6474;
    let t6476 = t6212 * t560;
    let t6477 = t6211 * t6476;
    let t6478 = t6475 * t6477;
    let t6480 = t565 * t6474;
    let t6481 = t6212 * t481;
    let t6482 = t6211 * t6481;
    let t6483 = t6480 * t6482;
    let t6486 = t133 * t2078 * t255;
    let t6487 = t546 * t6486;
    let t6490 = t565 * t6486;
    let t6493 = t2195 * t2168;
    (t6475, t6476, t6478, t6480, t6481, t6483, t6487, t6490, t6493)
}
