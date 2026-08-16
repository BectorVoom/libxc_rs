//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 893/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk893(t11177: f64, t11365: f64, t11366: f64, t1138: f64, t11400: f64, t11405: f64, t11409: f64, t11410: f64, t11415: f64, t11420: f64, t11421: f64, t11426: f64, t11429: f64, t11430: f64, t11434: f64, t11437: f64, t11441: f64, t11455: f64, t11472: f64, t1148: f64, t3327: f64, t3332: f64, t3352: f64, t3357: f64, t3360: f64, t3376: f64, t3401: f64, t436: f64) -> f64 {
    let t11473 = -0.10389515463408878255e3_f64 * t11365 * t11366 + 0.5848223622634646207e0_f64 * t1148 * t11400 + t11405 - t11409 + 3.0_f64 * t11410 * t1138 + 3.0_f64 * t3327 * t3352 + 0.96491876992155210402e2_f64 * t11415 * t3360 - 0.19298375398431042081e3_f64 * t11420 * t11421 + t11426 - t11429 - 0.35089341735807877242e1_f64 * t3376 * t11430 + 0.51947577317044391277e2_f64 * t3401 * t11434 - 6.0_f64 * t3332 * t11437 + 0.96491876992155210402e2_f64 * t3357 * t11441 - 0.310907e-1_f64 * t11455 * t436 - 0.19751673498613801407e-1_f64 * t11177 + t11472;
    t11473
}
