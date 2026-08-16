//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 572/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk572(t1222: f64, t2141: f64, t1225: f64, t2139: f64, t471: f64, t2145: f64, t225: f64, t1170: f64, t2148: f64, t2121: f64, t7284: f64, t477: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7343 = t2141 * t1222 / 2304.0_f64;
    let t7344 = t2139 * t1225;
    let t7345 = t471 * t7344;
    let t7351 = t2145 * t225;
    let t7359 = t1170 * t2148;
    let t7361 = 0.27415567780803773942e-2_f64 * t2121 * t7359;
    let t7362 = t7284 * t225;
    let t7363 = t477 * t491;
    (t7343, t7344, t7345, t7351, t7359, t7361, t7362, t7363)
}
