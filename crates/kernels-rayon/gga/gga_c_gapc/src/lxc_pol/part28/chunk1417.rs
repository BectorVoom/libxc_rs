//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1417/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1417(t35160: f64, t35162: f64, t35149: f64, t37210: f64, t37211: f64, t37212: f64, t37213: f64, t37214: f64, t37216: f64, t37217: f64, t37218: f64, t35169: f64) -> (f64, f64) {
    let t37219 = 0.33816362383187442026e-5_f64 * t35160;
    let t37220 = 0.80192315782160920384e-6_f64 * t35162;
    let t37221 = -t37210 - t37211 - t37212 - t37213 + t37214 - 0.64456181686737100543e-8_f64 * t35149 + t37216 + t37217 + t37218 + t37219 - t37220;
    let t37223 = 0.11984097313886885523e-6_f64 * t35169;
    (t37221, t37223)
}
