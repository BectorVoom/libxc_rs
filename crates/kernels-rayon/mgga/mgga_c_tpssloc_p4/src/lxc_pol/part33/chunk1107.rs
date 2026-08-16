//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1107/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1107(t212: f64, t252: f64, t6554: f64, t23171: f64, t23030: f64, t6563: f64, t1883: f64, t23012: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = 0.82246703342411321824e-2_f64 * t23230;
    let t23251 = t23030 * t6563;
    let t23252 = 0.26044789391763585244e-1_f64 * t23251;
    let t23261 = t23012 * t1883;
    let t23262 = 0.63969658155208805863e-1_f64 * t23261;
    let t23270 = t213 * t252 * t225;
    (t23228, t23229, t23231, t23252, t23262, t23270)
}
