//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1836/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1836(t2453: f64, t555: f64, t25898: f64, t1399: f64, t2438: f64, t25304: f64, t2482: f64, t7262: f64, t814: f64, t820: f64, t844: f64, t596: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94386 = t2438 * t1399;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94423 = t2482 * t7262 * t814;
    let t94429 = t820 * t7262 * t844;
    let t94443 = t2482 * t7269 * t596;
    (t94382, t94383, t94386, t94390, t94391, t94423, t94429, t94443)
}
