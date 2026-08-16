//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1013/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1013(t212: f64, t252: f64, t6554: f64, t23171: f64, t23030: f64, t6563: f64, t1883: f64, t23012: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23251 = t23030 * t6563;
    let t23261 = t23012 * t1883;
    let t23270 = t213 * t252 * t225;
    (t23228, t23229, t23230, t23251, t23261, t23270)
}
