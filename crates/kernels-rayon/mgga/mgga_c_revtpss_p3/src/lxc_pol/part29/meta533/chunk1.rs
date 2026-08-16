//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1865/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1865(t25431: f64, t95593: f64, t2646: f64, t26481: f64, t676: f64, t26482: f64, t93374: f64, t7385: f64, t9292: f64, t2772: f64, t689: f64, t7384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95594 = t25431 * t95593;
    let t95597 = t26481 * t676 * t2646;
    let t95598 = t25431 * t95597;
    let t95604 = t93374 * t26482;
    let t95607 = 0.17073386770573548589e-1_f64 * t9292 * t7385;
    let t95613 = t689 * t7384 * t2772;
    (t95594, t95597, t95598, t95604, t95607, t95613)
}
