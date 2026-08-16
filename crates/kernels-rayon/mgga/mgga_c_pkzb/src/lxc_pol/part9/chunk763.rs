//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 763/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk763(t5384: f64, t603: f64, t586: f64, t158: f64) -> (f64, f64, f64) {
    let t5385 = t5384 * t603;
    let t5387 = t586 * t586;
    let t5388 = 1.0_f64 / t5387;
    let t5389 = t158 * t5388;
    (t5385, t5387, t5389)
}
