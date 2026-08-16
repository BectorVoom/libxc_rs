//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 770/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk770(t7217: f64, t792: f64, t2847: f64, t498: f64, t6343: f64, t910: f64, t551: f64, t566: f64, t6512: f64, t924: f64, t133: f64, t255: f64, t2832: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7218 = t7217 * t792;
    let t7221 = t498 * t2847;
    let t7233 = t6343 * t910;
    let t7234 = t551 * t7233;
    let t7235 = t566 * t7234;
    let t7237 = t6512 * t924;
    let t7244 = t133 * t2832 * t255;
    (t7218, t7221, t7233, t7235, t7237, t7244)
}
