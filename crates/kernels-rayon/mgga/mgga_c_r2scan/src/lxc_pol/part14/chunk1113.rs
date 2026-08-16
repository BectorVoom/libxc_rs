//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1113/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1113(t10772: f64, t10810: f64, t2578: f64, t1577: f64, t2599: f64, t3308: f64, t574: f64, t7527: f64, t2096: f64, t2649: f64, t571: f64, t10769: f64) -> (f64, f64, f64, f64) {
    let t39400 = t10772 * t10810 * t2578;
    let t39403 = t1577 * t10810 * t2599;
    let t39406 = t574 * t3308 * t7527;
    let t39409 = t571 * t2649 * t2096;
    let t39410 = t39409 * t10769;
    (t39400, t39403, t39406, t39410)
}
