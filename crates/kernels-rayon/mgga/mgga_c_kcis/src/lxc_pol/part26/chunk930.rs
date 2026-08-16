//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 930/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk930(t1430: f64, t21106: f64, t21110: f64, t1437: f64, t21073: f64, t1330: f64, t21078: f64, t7164: f64, t733: f64, t7158: f64, t743: f64, t21020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21685 = t1430 * t21106;
    let t21688 = t1430 * t21110;
    let t21691 = t1437 * t21073;
    let t21694 = t1330 * t21078;
    let t21704 = t733 * t7164;
    let t21706 = t743 * t7158;
    let t21708 = t1430 * t21020;
    (t21685, t21688, t21691, t21694, t21704, t21706, t21708)
}
