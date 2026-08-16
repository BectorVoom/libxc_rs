//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2306/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2306<F: Float>(t28565: F, t6743: F, t23384: F, t28663: F, t23511: F, t5928: F, t100087: F, t11037: F, t1625: F, t23327: F, t23346: F, t23601: F, t23602: F, t23657: F, t23678: F, t25486: F, t25512: F, t28597: F, t28625: F, t28657: F, t3127: F, t6797: F, t6801: F, t82633: F, t82635: F, t83245: F, t884: F, t89094: F, t89104: F) -> F {
    let t100148 = t28565 * t6743;
    let t100163 = t23384 * t28663;
    let t100165 = t23511 * t5928;
    let t100176 = -t89094 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t100148 * t6801 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23657 * t28625 + F::cast_from(0.3289868133696452873e-1_f64) * t23601 * t23602 * t3127 * t1625 * t25486 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t100087 * t25512 - F::cast_from(0.54831135561607547883e-2_f64) * t100163 + F::cast_from(0.54831135561607547883e-2_f64) * t83245 * t100165 * t23678 * t884 + F::cast_from(0.18277045187202515961e-2_f64) * t82633 - t11037 * t28597 - F::cast_from(0.6092348395734171987e-3_f64) * t82635 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t28657 - F::cast_from(0.48738787165873375896e-2_f64) * t89104;
    t100176
}
