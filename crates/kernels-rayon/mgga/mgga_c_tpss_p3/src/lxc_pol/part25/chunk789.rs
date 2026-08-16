//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 789/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk789(t1196: f64, t1270: f64, t198: f64, t2292: f64, t2302: f64, t3205: f64, t3209: f64, t3213: f64, t3216: f64, t3281: f64, t3304: f64, t3307: f64, t3310: f64, t3391: f64, t509: f64, t5366: f64, t5371: f64, t5394: f64, t5451: f64, t5458: f64) -> f64 {
    let t5462 = t1270 * t198 * t509 * t5451 - t198 * t3205 * t509 * t5458 + 3.0_f64 * t1196 * t198 * t5366 + 6.0_f64 * t198 * t3391 * t5371 - t2292 + t2302 - t3209 + t3213 + t3216 + t3281 - t3304 + t3307 + t3310 + t5394;
    t5462
}
