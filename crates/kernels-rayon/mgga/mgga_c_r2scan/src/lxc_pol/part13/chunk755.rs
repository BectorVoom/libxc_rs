//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 755/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk755(t2148: f64, t6166: f64, t6165: f64, t1600: f64, t1629: f64, t2078: f64, t537: f64, t255: f64, t571: f64, t122: f64, t2111: f64, t409: f64, t57: f64) -> (f64, f64, f64, f64, f64) {
    let t6167 = t2148 * t6166;
    let t6168 = t6165 * t6167;
    let t6178 = t1600 * t1629;
    let t6180 = t537 * t2078;
    let t6182 = t571 * t6180 * t255;
    let t6188 = t2111 * t122;
    let t6189 = t409 * t57;
    (t6168, t6178, t6182, t6188, t6189)
}
