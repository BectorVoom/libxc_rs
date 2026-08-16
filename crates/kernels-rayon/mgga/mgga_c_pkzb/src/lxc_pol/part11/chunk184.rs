//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 184/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk184(t50: f64, t568: f64, t581: f64, t165: f64, t158: f64) -> (f64, f64, f64) {
    let t582 = t50 * t568;
    let t583 = t581 * t582;
    let t586 = t165 * t165;
    let t587 = 1.0_f64 / t586;
    let t588 = t158 * t587;
    (t583, t586, t588)
}
