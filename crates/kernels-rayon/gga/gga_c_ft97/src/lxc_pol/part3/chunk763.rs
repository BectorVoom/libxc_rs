//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 763/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk763(t15845: f64, t15878: f64, t409: f64, t64: f64, t11232: f64, t11233: f64, t15689: f64, t15797: f64, t15802: f64, t15806: f64, t15811: f64, t15812: f64, t15819: f64, t15822: f64, t15825: f64, t15829: f64, t1624: f64, t372: f64, t6426: f64, t7845: f64, t7877: f64, t7985: f64, t7989: f64) -> f64 {
    let t15879 = t15845 + t15878;
    let t15881 = t64 * t409 * t15879;
    let t15882 = 0.13519760450715832853e-3_f64 * t15797 * t7985 - 0.67598802253579164263e-4_f64 * t15797 * t7989 + 0.13784064983740990796e-3_f64 * t7845 * t15802 + 0.46509801892875584e-1_f64 * t7877 * t6426 * t15806 - 0.46509801892875584e-1_f64 * t15811 * t6426 * t15812 - 0.46509801892875584e-2_f64 * t11232 * t11233 * t15689 - 0.23254900946437792e-2_f64 * t1624 * t15819 - 0.279058811357253504e-2_f64 * t372 * t15822 + 0.46509801892875584e-2_f64 * t372 * t15825 + 0.23254900946437792e-1_f64 * t1624 * t15829 - t15881;
    t15882
}
