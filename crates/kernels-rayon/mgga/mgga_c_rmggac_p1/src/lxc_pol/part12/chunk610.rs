//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 610/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk610(t338: f64, t7667: f64, t118: f64, t4669: f64, t7193: f64, t5271: f64, t7199: f64, t5259: f64, t7205: f64, t3814: f64, t7710: f64, t5245: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7858 = t338 * t7667;
    let t7859 = t118 * t7858;
    let t7863 = t4669 * t7193;
    let t7865 = t5271 * t7199;
    let t7867 = t5259 * t7205;
    let t7869 = t3814 * t7710;
    let t7877 = t5245 * t645;
    (t7858, t7859, t7863, t7865, t7867, t7869, t7877)
}
