//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1147/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1147(t241: f64, t25260: f64, t820: f64, t72: f64, t7778: f64, t686: f64, t7064: f64, t25399: f64, t4481: f64, t1580: f64, t7014: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27261 = t820 * t25260 * t241;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    let t27325 = t25399 * t4481;
    let t27334 = t7014 * t1580;
    let t27335 = t689 * t27334;
    (t27261, t27278, t27279, t27280, t27325, t27334, t27335)
}
