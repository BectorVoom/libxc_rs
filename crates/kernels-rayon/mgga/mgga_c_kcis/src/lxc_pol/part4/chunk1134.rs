//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1134/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1134(t10414: f64, t10422: f64, t10424: f64, t10426: f64, t10428: f64, t1102: f64, t14313: f64, t14317: f64, t14321: f64, t14323: f64, t14327: f64, t14331: f64, t14339: f64, t14341: f64, t14343: f64, t14344: f64, t14348: f64, t14351: f64, t14355: f64, t14359: f64, t14363: f64, t14367: f64, t1697: f64, t3038: f64, t4587: f64) -> f64 {
    let t14372 = 0.26281718666666666666e-2_f64 * t10414 * t14313 - 0.19711289e-2_f64 * t10414 * t14317 + 0.32852148333333333333e-2_f64 * t14321 * t14323 - 0.21901432222222222222e-2_f64 * t14321 * t14327 - 0.19711289e-2_f64 * t10414 * t14331 + 0.13140859333333333333e-2_f64 * t10422 - 0.65704296666666666666e-3_f64 * t10424 + 0.43802864444444444444e-3_f64 * t10426 + 0.98556445e-3_f64 * t10428 - t14339 - t14341 + t14343 + 0.10950716111111111111e-2_f64 * t1102 * t14344 + 0.29201909629629629629e-2_f64 * t1102 * t14348 + 0.43802864444444444444e-2_f64 * t4587 * t14351 + 0.98556445e-3_f64 * t1102 * t14355 + 0.39422578e-2_f64 * t4587 * t14359 - 0.65704296666666666667e-3_f64 * t1102 * t14363 - 0.26281718666666666666e-2_f64 * t4587 * t14367 - 4.0_f64 * t3038 * t1697;
    t14372
}
