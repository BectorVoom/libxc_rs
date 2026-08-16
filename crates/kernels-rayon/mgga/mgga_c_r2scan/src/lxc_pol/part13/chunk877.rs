//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 877/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk877(t5109: f64, t7963: f64, t2559: f64, t7494: f64, t2124: f64, t2550: f64, t7944: f64, t1551: f64, t2526: f64, t277: f64, t495: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7964 = t5109 * t7963;
    let t7968 = 0.12805040077930161442e0_f64 * t7494 * t2559;
    let t7970 = t2124 * t2550 * t7944;
    let t7974 = t2124 * t2550 * t1551;
    let t7977 = t277 * t2526;
    let t7978 = t7977 * t495;
    let t7979 = t360 * t7978;
    (t7964, t7968, t7970, t7974, t7977, t7978, t7979)
}
