//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 746/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk746(t62: f64, t8538: f64, t8537: f64, t752: f64, t143: f64, t740: f64, t647: f64, t97: f64, t728: f64, t2440: f64, t2438: f64, t2459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8539 = t62 * t8538;
    let t8540 = t8537 * t8539;
    let t8541 = t752 * t8540;
    let t8543 = t143 * t740;
    let t8546 = t647 * t97;
    let t8547 = t8546 * t728;
    let t8556 = t2440 * t728;
    let t8557 = t2438 * t8556;
    let t8561 = t728 * t2459;
    (t8541, t8543, t8546, t8547, t8556, t8557, t8561)
}
