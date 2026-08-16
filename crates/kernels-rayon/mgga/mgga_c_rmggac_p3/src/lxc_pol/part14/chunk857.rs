//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 857/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk857(t1451: f64, t1979: f64, t1982: f64, t201: f64, t446: f64, t2283: f64, t7921: f64, t2185: f64, t8675: f64, t1997: f64, t1986: f64, t5277: f64, t675: f64) -> (f64, f64, f64, f64) {
    let t38963 = t446 * t1451 * t201 * t1979 * t1982;
    let t38965 = t7921 * t2283;
    let t38967 = t8675 * t2185;
    let t38968 = t38967 * t1997;
    let t38969 = 0.24829349937757072982e-4_f64 * t38968;
    let t38971 = t675 * t1986 * t5277;
    (t38963, t38965, t38969, t38971)
}
