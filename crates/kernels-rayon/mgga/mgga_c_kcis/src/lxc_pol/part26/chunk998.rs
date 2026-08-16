//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 998/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk998(t1539: f64, t7382: f64, t22664: f64, t4293: f64, t6010: f64, t4249: f64, t7335: f64, t2051: f64, t5935: f64, t1929: f64, t4254: f64, t6029: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22672 = t7382 * t1539;
    let t22674 = t4293 * t22664;
    let t22675 = t6010 * t22674;
    let t22677 = t4249 * t7335;
    let t22679 = t2051 * t5935;
    let t22681 = t4254 * t1929;
    let t22682 = t22681 * t6029;
    (t22672, t22674, t22675, t22677, t22679, t22682)
}
