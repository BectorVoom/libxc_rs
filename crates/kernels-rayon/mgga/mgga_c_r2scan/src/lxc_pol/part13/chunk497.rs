//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 497/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk497(t2252: f64, t552: f64, t551: f64, t2196: f64, t2198: f64, t2204: f64, t2210: f64, t2212: f64, t2216: f64, t2220: f64, t2223: f64, t2225: f64, t2231: f64, t2233: f64, t2236: f64, t527: f64, t549: f64, t562: f64) -> (f64, f64, f64) {
    let t2253 = t552 * t2252;
    let t2254 = t551 * t2253;
    let t2257 = 0.5200933044032561138e0_f64 * t2196 * t2198 - 0.11643651550782197811e-1_f64 * t2204 - 0.34930954652346593434e-1_f64 * t2210 + 0.54878743191129263322e-2_f64 * t2212 - 0.19514881078765566037e-1_f64 * t2216 + 0.69345773920434148506e0_f64 * t2220 + 0.16463622957338778997e0_f64 * t2223 * t2225 + t2231 - 0.54878743191129263322e-1_f64 * t527 * t2233 - 0.86682217400542685632e-1_f64 * t2236 * t562 - 0.43341108700271342816e-1_f64 * t549 * t2254;
    (t2253, t2254, t2257)
}
