//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 622/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk622<F: Float>(t3559: F, t64: F, t35: F, t1227: F, t3494: F, t3505: F, t3508: F, t3513: F, t3515: F, t3517: F, t3521: F, t3523: F, t3525: F, t3526: F, t3531: F, t3534: F, t360: F, t63: F) -> (F, F, F) {
    let t3560 = t64 * t3559;
    let t3561 = t35 * t3560;
    let t3564 = F::new(17.62848) * t63 * t3494 * t1227 - t3505 + t3508 + t3513 - t3515 - t3517 - t3521 - t3523 + t3525 + F::new(9.0) / F::new(2.0) * t360 * t35 * t3526 - F::new(2.0) / F::new(3.0) * t3531 + t3534 / F::new(2.0) - t360 * t3561 / F::new(2.0);
    (t3560, t3561, t3564)
}
