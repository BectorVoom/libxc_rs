//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1004/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1004(t269: f64, t2832: f64, t1060: f64, t783: f64, t2201: f64, t3324: f64, t3613: f64, t2207: f64, t3328: f64, t3336: f64, t3602: f64, t2719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11780 = t2832 * t269;
    let t11782 = t783 * t11780 * t1060;
    let t11785 = t2201 * t3613 * t3324;
    let t11788 = t2207 * t3613 * t3328;
    let t11791 = t2201 * t3336 * t3602;
    let t11793 = t1060 * t2719;
    (t11780, t11782, t11785, t11788, t11791, t11793)
}
