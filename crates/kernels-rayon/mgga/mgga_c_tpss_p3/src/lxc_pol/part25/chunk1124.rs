//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1124/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1124(t15463: f64, t408: f64, t4192: f64, t4207: f64, t3154: f64, t5297: f64, t294: f64, t5156: f64, t1091: f64, t12009: f64, t1551: f64, t1151: f64, t15232: f64, t15355: f64, t15356: f64, t15361: f64, t15363: f64, t15365: f64, t15411: f64, t15413: f64, t15417: f64, t15421: f64, t15426: f64, t15443: f64, t15446: f64, t15448: f64, t4023: f64) -> (f64, f64, f64, f64, f64) {
    let t15465 = 0.621814e-1_f64 * t15463 * t408;
    let t15467 = 0.34631718211362927517e2_f64 * t4192 * t4207;
    let t15468 = t5297 * t3154;
    let t15471 = t294 * t5156;
    let t15473 = 0.5848223622634646207e0_f64 * t15471 * t1091;
    let t15475 = 0.11696447245269292414e1_f64 * t12009 * t1551;
    let t15476 = 2.0_f64 * t1151 * t15356 * t4023 - t1151 * t15468 * t4023 - t15232 - t15355 + t15361 - t15363 + t15365 + t15411 + t15413 - t15417 + t15421 - t15426 + t15443 + t15446 + t15448 - t15465 - t15467 - t15473 - t15475;
    (t15465, t15467, t15473, t15475, t15476)
}
