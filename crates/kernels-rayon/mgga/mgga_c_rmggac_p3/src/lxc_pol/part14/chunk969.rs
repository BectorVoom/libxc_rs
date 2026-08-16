//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 969/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk969(t40564: f64, t2320: f64, t35151: f64, t34847: f64, t8668: f64, t1525: f64, t236: f64, t498: f64, t7230: f64, t7231: f64, t333: f64, t8957: f64) -> (f64, f64, f64, f64, f64) {
    let t40565 = 0.24829349937757072982e-4_f64 * t40564;
    let t40566 = t35151 * t2320;
    let t40567 = 0.24829349937757072982e-4_f64 * t40566;
    let t40568 = t34847 * t8668;
    let t40573 = t7230 * t7231 * t236 * t1525 * t498;
    let t40575 = t8957 * t333;
    (t40565, t40567, t40568, t40573, t40575)
}
