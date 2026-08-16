//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 966/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk966(t47243: f64, t7427: f64, t7573: f64, t12223: f64, t1445: f64, t2530: f64, t813: f64, t13870: f64, t2089: f64, t2087: f64, t723: f64, t13865: f64, t4614: f64) -> (f64, f64, f64, f64) {
    let t47245 = t7427 * t7573 * t47243;
    let t47255 = t813 * t1445 * t12223 * t2530;
    let t47257 = t2089 * t13870;
    let t47261 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t47257 * t723;
    let t47263 = t2087 * t4614 * t13865;
    (t47245, t47255, t47261, t47263)
}
