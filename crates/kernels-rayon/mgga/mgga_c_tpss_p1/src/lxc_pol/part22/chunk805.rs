//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 805/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk805(t1196: f64, t1270: f64, t198: f64, t2281: f64, t2285: f64, t3182: f64, t3189: f64, t3194: f64, t3196: f64, t4357: f64, t4359: f64, t4379: f64, t4397: f64, t4428: f64, t4429: f64, t4431: f64, t4433: f64, t4437: f64, t4519: f64, t509: f64) -> f64 {
    let t4523 = t1270 * t198 * t4519 * t509 + 3.0_f64 * t1196 * t198 * t4397 - t2281 - t2285 - t3182 + t3189 + t3194 - t3196 + t4357 - t4359 + t4379 - t4428 - t4429 + t4431 + t4433 - t4437;
    t4523
}
