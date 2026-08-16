//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 814/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk814(t6343: f64, t910: f64, t551: f64, t566: f64, t6512: f64, t924: f64, t552: f64, t7088: f64, t133: f64, t255: f64, t2832: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7233 = t6343 * t910;
    let t7234 = t551 * t7233;
    let t7235 = t566 * t7234;
    let t7237 = t6512 * t924;
    let t7239 = t552 * t7088;
    let t7240 = t551 * t7239;
    let t7244 = t133 * t2832 * t255;
    let t7245 = t546 * t7244;
    (t7233, t7235, t7237, t7239, t7240, t7244, t7245)
}
