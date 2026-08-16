//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 660/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk660(t2471: f64, t787: f64, t206: f64, t242: f64, t240: f64, t72: f64, t2394: f64, t828: f64, t225: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2473 = 0.13009920719177044025e-1_f64 * t787 * t2471;
    let t2475 = 1.0_f64 / t242 / t206;
    let t2476 = t240 * t2475;
    let t2477 = t2476 * t72;
    let t2479 = t2477 * t828 * t2394;
    let t2482 = t786 * t225;
    (t2473, t2475, t2476, t2477, t2479, t2482)
}
