//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 484/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk484(t3481: f64, t491: f64, t1190: f64, t1235: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t3408: f64, t3410: f64, t3413: f64, t3417: f64, t3421: f64, t3425: f64) -> (f64, f64, f64, f64, f64) {
    let t3482 = t3481 * t491;
    let t3484 = t1190 * t1235;
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    let t3493 = -t3258 + t3261 - t3268 + t3310 + t3318 + t3408 + t3410 - t3413 + t3417 - t3421 - t3425;
    (t3482, t3484, t3487, t3490, t3493)
}
