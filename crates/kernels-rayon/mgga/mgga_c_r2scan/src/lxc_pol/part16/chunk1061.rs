//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1061/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1061(t2096: f64, t265: f64, t267: f64, t37625: f64, t546: f64, t6476: f64, t565: f64, t6481: f64, t2111: f64, t409: f64, t5148: f64, t2157: f64, t625: f64) -> (f64, f64, f64, f64) {
    let t37628 = t2096 * t265 * t267;
    let t37630 = t546 * t37625 * t37628 * t6476;
    let t37634 = t565 * t37625 * t37628 * t6481;
    let t37637 = t2111 * t409 * t5148;
    let t37638 = t2157 * t625;
    (t37630, t37634, t37637, t37638)
}
