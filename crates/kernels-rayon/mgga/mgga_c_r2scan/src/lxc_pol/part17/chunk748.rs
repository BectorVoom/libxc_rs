//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 748/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk748(t495: f64, t551: f64, t6343: f64, t574: f64, t536: f64, t252: f64, t255: f64, t571: f64, t113: f64, t1569: f64, t2145: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6345 = t551 * t6343 * t495;
    let t6346 = t574 * t6345;
    let t6358 = t536 * t536;
    let t6359 = 1.0_f64 / t6358;
    let t6360 = t6359 * t252;
    let t6362 = t571 * t6360 * t255;
    let t6363 = t1569 * t113;
    let t6394 = t2145 * t774;
    (t6346, t6359, t6360, t6362, t6363, t6394)
}
