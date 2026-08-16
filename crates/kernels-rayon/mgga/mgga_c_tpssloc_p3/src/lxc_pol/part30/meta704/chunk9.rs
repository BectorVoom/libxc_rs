//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2306/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2306(t28565: f64, t6743: f64, t23384: f64, t28663: f64, t23511: f64, t5928: f64, t100087: f64, t11037: f64, t1625: f64, t23327: f64, t23346: f64, t23601: f64, t23602: f64, t23657: f64, t23678: f64, t25486: f64, t25512: f64, t28597: f64, t28625: f64, t28657: f64, t3127: f64, t6797: f64, t6801: f64, t82633: f64, t82635: f64, t83245: f64, t884: f64, t89094: f64, t89104: f64) -> f64 {
    let t100148 = t28565 * t6743;
    let t100163 = t23384 * t28663;
    let t100165 = t23511 * t5928;
    let t100176 = -t89094 - 0.82246703342411321825e-2_f64 * t6797 * t100148 * t6801 - 0.16449340668482264365e-1_f64 * t6797 * t23657 * t28625 + 0.3289868133696452873e-1_f64 * t23601 * t23602 * t3127 * t1625 * t25486 + 0.10966227112321509577e-1_f64 * t23327 * t100087 * t25512 - 0.54831135561607547883e-2_f64 * t100163 + 0.54831135561607547883e-2_f64 * t83245 * t100165 * t23678 * t884 + 0.18277045187202515961e-2_f64 * t82633 - t11037 * t28597 - 0.6092348395734171987e-3_f64 * t82635 + 0.21932454224643019153e-1_f64 * t23346 * t28657 - 0.48738787165873375896e-2_f64 * t89104;
    t100176
}
