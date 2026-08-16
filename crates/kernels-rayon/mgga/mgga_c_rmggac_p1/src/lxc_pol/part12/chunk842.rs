//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 842/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk842(t4601: f64, t8551: f64, t2060: f64, t31125: f64, t903: f64, t321: f64, t8700: f64, t262: f64, t7198: f64, t7345: f64, t8349: f64, t1665: f64, t2010: f64, t7359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38739 = t4601 * t8551;
    let t38742 = t903 * t2060 * t31125;
    let t38745 = t8700 * t321;
    let t38746 = t262 * t38745;
    let t38747 = t7198 * t38746;
    let t38749 = t7345 * t8349;
    let t38752 = t2010 * t7359 * t1665;
    (t38739, t38742, t38745, t38746, t38747, t38749, t38752)
}
