//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1053/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1053<F: Float>(t10631: F, t1153: F, t14205: F, t14291: F, t14295: F, t15001: F, t15004: F, t15008: F, t15009: F, t15013: F, t15016: F, t15019: F, t15023: F, t15026: F, t15037: F, t15040: F, t15046: F, t15047: F, t15050: F, t3381: F, t368: F, t5111: F, t5122: F, t5133: F, t86: F) -> (F,) {
    let t15053 = 0.53062222222222222222e-1 * t1153 * t15001 + 0.26531111111111111111e0 * t5133 * t15004 + 0.21224888888888888888e0 * t15008 * t15009 + 0.10612444444444444444e0 * t5133 * t15013 + 0.53062222222222222222e-1 * t5133 * t15016 - 0.44218518518518518518e-1 * t5133 * t15019 - 0.11791604938271604938e0 * t5133 * t15023 - 0.17687407407407407407e0 * t15008 * t15026 - 0.9286875e-2 * t3381 * t14291 - 0.371475e-1 * t5122 * t14205 + 0.24765e-1 * t5122 * t14295 + 0.17687407407407407407e-1 * t10631 - 0.9286875e-2 * t5111 * t15037 - 0.39796666666666666666e-1 * t86 * t368 * t15040 - t15046 - 0.15918666666666666667e0 * t5133 * t15047 + 0.371475e-1 * t3381 * t15050;
    (t15053,)
}
