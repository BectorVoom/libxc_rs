//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2066/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2066(t3469: f64, t52: f64, t24682: f64, t460: f64, t3475: f64, t11702: f64, t7339: f64, t24684: f64, t27634: f64, t1210: f64, t24654: f64, t24721: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86197 = t52 * t3469;
    let t86199 = t24682 * t86197 * t460;
    let t86202 = t52 * t3475;
    let t86204 = t24682 * t86202 * t460;
    let t86228 = t7339 * t11702;
    let t86234 = t27634 * t24684;
    let t86248 = t24721 * t1210 * t24654;
    (t86197, t86199, t86202, t86204, t86228, t86234, t86248)
}
