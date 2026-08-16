//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 498/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk498<F: Float>(t2252: F, t552: F, t551: F, t2196: F, t2198: F, t2204: F, t2210: F, t2212: F, t2216: F, t2220: F, t2223: F, t2225: F, t2231: F, t2233: F, t2236: F, t527: F, t549: F, t562: F) -> (F, F, F) {
    let t2253 = t552 * t2252;
    let t2254 = t551 * t2253;
    let t2257 = F::cast_from(0.5200933044032561138e0_f64) * t2196 * t2198 - F::cast_from(0.11643651550782197811e-1_f64) * t2204 - F::cast_from(0.34930954652346593434e-1_f64) * t2210 + F::cast_from(0.54878743191129263322e-2_f64) * t2212 - F::cast_from(0.19514881078765566037e-1_f64) * t2216 + F::cast_from(0.69345773920434148506e0_f64) * t2220 + F::cast_from(0.16463622957338778997e0_f64) * t2223 * t2225 + t2231 - F::cast_from(0.54878743191129263322e-1_f64) * t527 * t2233 - F::cast_from(0.86682217400542685632e-1_f64) * t2236 * t562 - F::cast_from(0.43341108700271342816e-1_f64) * t549 * t2254;
    (t2253, t2254, t2257)
}
