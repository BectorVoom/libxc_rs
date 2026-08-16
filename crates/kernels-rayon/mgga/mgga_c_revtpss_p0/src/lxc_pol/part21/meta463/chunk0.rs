//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2004/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2004(t14676: f64, t4364: f64, t837: f64, t2646: f64, t4365: f64, t136: f64, t243: f64, t220: f64) -> (f64, f64, f64) {
    let t14678 = t4364 * t14676 * t837;
    let t14682 = t4364 * t4365 * t2646;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    (t14678, t14682, t14686)
}
