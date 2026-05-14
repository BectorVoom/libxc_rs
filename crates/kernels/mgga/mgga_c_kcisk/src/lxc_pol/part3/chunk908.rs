//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 908/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk908<F: Float>(t14345: F, t14348: F, t14351: F, t14354: F, t14357: F, t14359: F, t14361: F, t14363: F, t14368: F, t14371: F, t14377: F, t14381: F, t14388: F, t14391: F, t14394: F, t14400: F, t14402: F, t14514: F, t14516: F, t14519: F, t14522: F, t14525: F, t14529: F, t14532: F, t14536: F, t14538: F, t14541: F, t14543: F, t14548: F, t14552: F) -> (F, F) {
    let t15132 = 0.80937499999999999999e-1 * t14345 - 0.13489583333333333333e-1 * t14348 - 0.13669444444444444444e1 * t14351 + 0.375e0 * t14354 - 0.60703125e-1 * t14357 + 0.1875e0 * t14359 - 0.40468749999999999999e-1 * t14361 + 0.15e1 * t14363 - 0.5625e0 * t14368 + 0.18333333333333333333e1 * t14371 + 0.29976851851851851851e-2 * t14377 - 0.42777777777777777778e1 * t14381 + 0.25060648148148148148e1 * t14388 - 0.1875e0 * t14391 + 0.10252083333333333334e1 * t14394;
    let t15149 = -0.28125e0 * t14400 + 0.303515625e-1 * t14402 + 0.9375e-1 * t14514 - 0.13489583333333333333e-1 * t14516 + 0.40468749999999999999e-1 * t14519 - 0.62499999999999999999e-1 * t14522 + 0.75e0 * t14525 + 0.625e-1 * t14529 - 0.60703125e-1 * t14532 - 0.13489583333333333333e-1 * t14536 - 1.0 * t14538 - 0.50000000000000000001e0 * t14541 + 0.1875e0 * t14543 + 0.60703125e-1 * t14548 - 0.101171875e-1 * t14552;
    (t15132, t15149)
}
