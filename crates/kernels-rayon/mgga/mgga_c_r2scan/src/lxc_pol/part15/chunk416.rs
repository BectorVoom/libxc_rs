//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 416/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk416(t1691: f64, t717: f64, t720: f64, t61: f64, t424: f64, t697: f64, t1678: f64, t614: f64, t22: f64, t263: f64, t124: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    let t1709 = 0.19263893255070628431e1_f64 * t61 * t1707;
    let t1710 = t424 * t697;
    let t1712 = t1678 * t614;
    let t1713 = t22 * t263;
    let t1714 = t1712 * t1713;
    let t1716 = t5 * t124;
    (t1707, t1709, t1710, t1712, t1713, t1714, t1716)
}
