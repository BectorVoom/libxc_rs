//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1434/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1434<F: Float>(t13687: F, t3663: F, t9566: F, t4524: F, t531: F, t2923: F, t3785: F, t9531: F, t9448: F, t9625: F, t4544: F, t11597: F, t11605: F, t11608: F, t11611: F, t26113: F, t2842: F, t5198: F, t9458: F, t9597: F, t9598: F, t9603: F, t9604: F, t9621: F, t9737: F) -> (F, F, F, F, F, F) {
    let t31106 = t3663 * t13687;
    let t31114 = t3663 * t9566;
    let t31119 = t531 * t4524;
    let t31120 = t31119 * t2923;
    let t31123 = t3785 * t9531;
    let t31126 = t9625 * t9448;
    let t31136 = t531 * t4544 * t2923;
    let t31139 = -F::cast_from(11200.0_f64) / F::cast_from(9.0_f64) * t9621 * t31106 + F::cast_from(800.0_f64) / F::cast_from(9.0_f64) * t5198 * t9597 * t9604 - F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t9598 * t31106 + F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t9598 * t31114 - F::cast_from(1600.0_f64) / F::cast_from(9.0_f64) * t9603 * t31106 - F::cast_from(2016.0_f64) * t26113 * t31120 + F::cast_from(24.0_f64) * t31123 * t11597 - F::cast_from(2016.0_f64) * t26113 * t31126 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t11605 * t2842 - F::cast_from(1600.0_f64) / F::cast_from(9.0_f64) * t11608 * t9458 + F::cast_from(8000.0_f64) / F::cast_from(9.0_f64) * t11611 * t9458 + F::cast_from(252.0_f64) * t9737 * t31136;
    (t31106, t31114, t31120, t31126, t31136, t31139)
}
