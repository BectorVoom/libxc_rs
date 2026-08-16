//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 915/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk915(t10143: f64, t7109: f64, t2047: f64, t2678: f64, t225: f64, t24200: f64, t24237: f64, t24235: f64, t25: f64, t40772: f64, t606: f64, t254: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84800 = t7109 * t10143;
    let t84842 = t2047 * t2678;
    let t85079 = t24200 * t225;
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t86716 = t40772 * t25;
    let t86770 = t10143 * t606;
    let t87013 = t853 * t254;
    (t84800, t84842, t85079, t85146, t85152, t86716, t86770, t87013)
}
