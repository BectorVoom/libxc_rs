//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1628/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1628(t23253: f64, t6562: f64, t225: f64, t258: f64, t2710: f64, t214: f64, t1880: f64, t1883: f64, t23012: f64, t23237: f64, t6572: f64, t213: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23254 = t6562 * t23253;
    let t23257 = t2710 * t225 * t258;
    let t23258 = t214 * t23257;
    let t23259 = t1880 * t23258;
    let t23261 = t23012 * t1883;
    let t23265 = t23237 * t6572;
    let t23266 = t1880 * t23265;
    let t23270 = t213 * t252 * t225;
    (t23254, t23257, t23258, t23259, t23261, t23265, t23266, t23270)
}
