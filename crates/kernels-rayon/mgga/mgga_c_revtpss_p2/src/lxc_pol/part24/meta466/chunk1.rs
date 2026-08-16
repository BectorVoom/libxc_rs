//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1441/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1441(t18796: f64, t2465: f64, t2470: f64, t18811: f64, t2435: f64, t18825: f64, t2453: f64, t2458: f64, t6042: f64, t2439: f64, t2440: f64, t6049: f64) -> (f64, f64, f64, f64, f64) {
    let t61355 = t2465 * t18796 * t2470;
    let t61361 = t2435 * t18811;
    let t61367 = t2435 * t18825;
    let t61371 = t2453 * t6042 * t2458;
    let t61397 = t2439 * t2440 * t6049;
    (t61355, t61361, t61367, t61371, t61397)
}
