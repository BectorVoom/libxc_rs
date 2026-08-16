//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1063/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1063(t26611: f64, t7580: f64, t209: f64, t2410: f64, t7590: f64, t7589: f64, t698: f64, t2389: f64, t700: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26612 = t7580 * t26611;
    let t26615 = t209 * t7590 * t2410;
    let t26616 = t7589 * t26615;
    let t26618 = t7589 * t26611;
    let t26620 = t209 * t698;
    let t26623 = t26620 * t2389 * t700 * t705;
    (t26612, t26615, t26616, t26618, t26620, t26623)
}
