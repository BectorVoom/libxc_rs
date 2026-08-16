//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 839/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk839(t2060: f64, t31043: f64, t903: f64, t1990: f64, t38472: f64, t7364: f64, t8576: f64, t7367: f64, t16156: f64, t8508: f64, t3351: f64, t511: f64, t5226: f64, t9188: f64) -> (f64, f64, f64, f64, f64) {
    let t38695 = t903 * t2060 * t31043;
    let t38699 = t38472 * t1990;
    let t38701 = t8576 * t7364;
    let t38702 = t38701 * t7367;
    let t38704 = t16156 * t8508;
    let t38705 = 0.17877131955185092547e-3_f64 * t38704;
    let t38708 = t3351 * t9188 * t511 * t5226;
    (t38695, t38699, t38702, t38705, t38708)
}
