//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1841/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1841(t2453: f64, t555: f64, t25898: f64, t1399: f64, t2438: f64, t25304: f64, t1444: f64, t543: f64, t268: f64, t4102: f64, t4057: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94386 = t2438 * t1399;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94396 = t543 * t1444;
    let t94398 = t268 * t4102 * t94396;
    let t94403 = t676 * t4057;
    (t94382, t94383, t94386, t94390, t94391, t94398, t94403)
}
