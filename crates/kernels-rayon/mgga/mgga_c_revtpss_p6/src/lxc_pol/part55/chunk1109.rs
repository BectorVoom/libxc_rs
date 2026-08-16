//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1109/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1109(t2108: f64, t34033: f64, t34168: f64, t34191: f64, t34193: f64, t34195: f64, t34198: f64, t34203: f64, t34244: f64, t34250: f64, t34253: f64, t34399: f64, t7359: f64, t8109: f64, t8158: f64, t8463: f64, t8764: f64) -> f64 {
    let t34795 = t2108 * t34399 - 2.0_f64 * t7359 * t8158 + t8109 * t8764 + t34033 - t34168 + t34191 - t34193 - t34195 - t34198 + t34203 + t34244 - t34250 - t34253 - t8463;
    t34795
}
