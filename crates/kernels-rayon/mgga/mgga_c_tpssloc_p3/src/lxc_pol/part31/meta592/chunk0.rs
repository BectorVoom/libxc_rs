//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1837/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1837(t81375: f64, t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t22573: f64, t7684: f64, t27240: f64, t580: f64, t1395: f64, t7961: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91496 = 0.25587863262083522346e0_f64 * t81375;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91655 = t7684 * t22573;
    let t91830 = 2.0_f64 * t27240 * t580;
    let t91832 = 2.0_f64 * t1395 * t7961;
    (t91496, t91531, t91548, t91655, t91830, t91832)
}
