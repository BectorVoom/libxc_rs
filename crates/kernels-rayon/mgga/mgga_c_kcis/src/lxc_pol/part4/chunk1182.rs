//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1182/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1182(t15036: f64, t4606: f64, t13618: f64, t304: f64, t238: f64, t5158: f64, t86: f64, t13495: f64, t5142: f64, t1728: f64, t10631: f64, t1153: f64, t14205: f64, t14291: f64, t14295: f64, t15001: f64, t15004: f64, t15008: f64, t15009: f64, t15013: f64, t15016: f64, t15019: f64, t15023: f64, t15026: f64, t3381: f64, t368: f64, t5111: f64, t5122: f64, t5133: f64) -> f64 {
    let t15037 = t4606 * t15036;
    let t15040 = t304 * t13618;
    let t15046 = 0.53062222222222222222e-1_f64 * t86 * t238 * t5158;
    let t15047 = t5142 * t13495;
    let t15050 = t1728 * t15036;
    let t15053 = 0.53062222222222222222e-1_f64 * t1153 * t15001 + 0.26531111111111111111e0_f64 * t5133 * t15004 + 0.21224888888888888888e0_f64 * t15008 * t15009 + 0.10612444444444444444e0_f64 * t5133 * t15013 + 0.53062222222222222222e-1_f64 * t5133 * t15016 - 0.44218518518518518518e-1_f64 * t5133 * t15019 - 0.11791604938271604938e0_f64 * t5133 * t15023 - 0.17687407407407407407e0_f64 * t15008 * t15026 - 0.9286875e-2_f64 * t3381 * t14291 - 0.371475e-1_f64 * t5122 * t14205 + 0.24765e-1_f64 * t5122 * t14295 + 0.17687407407407407407e-1_f64 * t10631 - 0.9286875e-2_f64 * t5111 * t15037 - 0.39796666666666666666e-1_f64 * t86 * t368 * t15040 - t15046 - 0.15918666666666666667e0_f64 * t5133 * t15047 + 0.371475e-1_f64 * t3381 * t15050;
    t15053
}
