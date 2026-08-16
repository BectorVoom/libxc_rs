//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 700/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk700(t507: f64, t541: f64, t1196: f64, t1270: f64, t198: f64, t2292: f64, t2302: f64, t3213: f64, t3216: f64, t3234: f64, t3245: f64, t3281: f64, t3299: f64, t3302: f64, t3304: f64, t3307: f64, t3310: f64, t3312: f64, t3387: f64, t509: f64) -> f64 {
    let t3391 = t507 * t541;
    let t3395 = t1270 * t198 * t3387 * t509 + 3.0_f64 * t1196 * t198 * t3234 + 6.0_f64 * t198 * t3245 * t3391 - t2292 + t2302 + t3213 - t3216 + t3281 + t3299 + t3302 + t3304 + t3307 + t3310 + t3312;
    t3395
}
