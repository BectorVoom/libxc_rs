//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1023/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1023(t14345: f64, t14348: f64, t14351: f64, t14354: f64, t14357: f64, t14359: f64, t14361: f64, t14363: f64, t14368: f64, t14371: f64, t14377: f64, t14381: f64, t14388: f64, t14391: f64, t14394: f64) -> f64 {
    let t15132 = 0.80937499999999999999e-1_f64 * t14345 - 0.13489583333333333333e-1_f64 * t14348 - 0.13669444444444444444e1_f64 * t14351 + 0.375e0_f64 * t14354 - 0.60703125e-1_f64 * t14357 + 0.1875e0_f64 * t14359 - 0.40468749999999999999e-1_f64 * t14361 + 0.15e1_f64 * t14363 - 0.5625e0_f64 * t14368 + 0.18333333333333333333e1_f64 * t14371 + 0.29976851851851851851e-2_f64 * t14377 - 0.42777777777777777778e1_f64 * t14381 + 0.25060648148148148148e1_f64 * t14388 - 0.1875e0_f64 * t14391 + 0.10252083333333333334e1_f64 * t14394;
    t15132
}
