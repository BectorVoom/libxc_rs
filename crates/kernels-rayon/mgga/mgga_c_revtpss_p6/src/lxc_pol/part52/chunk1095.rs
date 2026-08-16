//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1095/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1095(t34216: f64, t34240: f64, t532: f64, t1450: f64, t2014: f64, t1519: f64, t1843: f64, t1932: f64, t2089: f64, t2108: f64, t32389: f64, t33913: f64, t34168: f64, t34188: f64, t34191: f64, t34193: f64, t34195: f64, t34198: f64, t34203: f64, t508: f64, t7725: f64, t8065: f64, t8109: f64, t8568: f64, t8627: f64) -> (f64, f64, f64, f64) {
    let t34241 = t34216 + t34240;
    let t34242 = t532 * t34241;
    let t34243 = t34242 * t1450;
    let t34244 = t2014 * t34243;
    let t34245 = -2.0_f64 * t1519 * t32389 - t1843 * t8627 - t1932 * t8065 - t2089 * t7725 + t2108 * t33913 - t34188 * t508 + t8109 * t8568 - t34168 + t34191 - t34193 - t34195 - t34198 + t34203 + t34244;
    (t34241, t34242, t34243, t34245)
}
