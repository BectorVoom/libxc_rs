//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 838/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk838(t2526: f64, t481: f64, t506: f64, t529: f64, t551: f64, t6343: f64, t921: f64, t574: f64, t2145: f64, t978: f64, t146: f64) -> (f64, f64, f64, f64, f64) {
    let t7591 = t2526 * t481;
    let t7593 = t529 * t506 * t7591;
    let t7597 = t551 * t6343 * t921;
    let t7598 = t574 * t7597;
    let t7600 = t2145 * t978;
    let t7601 = t146 * t7600;
    (t7591, t7593, t7598, t7600, t7601)
}
