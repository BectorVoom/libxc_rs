//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2600/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600(t18825: f64, t2435: f64, t2453: f64, t2458: f64, t6042: f64, t18785: f64, t689: f64, t779: f64, t18316: f64, t887: f64, t2439: f64, t2440: f64, t6049: f64) -> (f64, f64, f64, f64, f64) {
    let t61367 = t2435 * t18825;
    let t61371 = t2453 * t6042 * t2458;
    let t61378 = t689 * t779 * t18785;
    let t61385 = t689 * t18316 * t887;
    let t61397 = t2439 * t2440 * t6049;
    (t61367, t61371, t61378, t61385, t61397)
}
