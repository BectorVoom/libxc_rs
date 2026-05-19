//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1134/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1134<F: Float>(t10414: F, t10422: F, t10424: F, t10426: F, t10428: F, t1102: F, t14313: F, t14317: F, t14321: F, t14323: F, t14327: F, t14331: F, t14339: F, t14341: F, t14343: F, t14344: F, t14348: F, t14351: F, t14355: F, t14359: F, t14363: F, t14367: F, t1697: F, t3038: F, t4587: F) -> F {
    let t14372 = F::cast_from(0.26281718666666666666e-2_f64) * t10414 * t14313 - F::new(0.19711289e-2) * t10414 * t14317 + F::cast_from(0.32852148333333333333e-2_f64) * t14321 * t14323 - F::cast_from(0.21901432222222222222e-2_f64) * t14321 * t14327 - F::new(0.19711289e-2) * t10414 * t14331 + F::cast_from(0.13140859333333333333e-2_f64) * t10422 - F::cast_from(0.65704296666666666666e-3_f64) * t10424 + F::cast_from(0.43802864444444444444e-3_f64) * t10426 + F::new(0.98556445e-3) * t10428 - t14339 - t14341 + t14343 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t14344 + F::cast_from(0.29201909629629629629e-2_f64) * t1102 * t14348 + F::cast_from(0.43802864444444444444e-2_f64) * t4587 * t14351 + F::new(0.98556445e-3) * t1102 * t14355 + F::new(0.39422578e-2) * t4587 * t14359 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t14363 - F::cast_from(0.26281718666666666666e-2_f64) * t4587 * t14367 - F::new(4.0) * t3038 * t1697;
    t14372
}
