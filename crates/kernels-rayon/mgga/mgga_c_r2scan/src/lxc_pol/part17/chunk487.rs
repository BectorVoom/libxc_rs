//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 487/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk487(t2550: f64, t2551: f64, t2124: f64, t129: f64, t524: f64, t525: f64, t495: f64, t277: f64, t938: f64) -> (f64, f64, f64, f64) {
    let t2552 = t2550 * t2551;
    let t2553 = t2124 * t2552;
    let t2557 = t524 * t525 * t129;
    let t2558 = t2550 * t495;
    let t2559 = t2124 * t2558;
    let t2562 = t277 * t938;
    (t2553, t2557, t2559, t2562)
}
