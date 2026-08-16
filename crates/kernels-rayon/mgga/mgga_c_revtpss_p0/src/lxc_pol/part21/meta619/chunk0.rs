//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2374/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374(t10832: f64, t10845: f64, t820: f64, t823: f64, t9948: f64, t839: f64, t10639: f64, t221: f64, t2484: f64, t2485: f64, t10820: f64, t2652: f64) -> (f64, f64, f64, f64, f64) {
    let t40357 = t10845 * t10832;
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40365 = t2484 * t2485 * t221 * t10639;
    let t40367 = t2652 * t10820;
    (t40357, t40360, t40361, t40365, t40367)
}
