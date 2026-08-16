//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1434/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1434(t13687: f64, t3663: f64, t9566: f64, t4524: f64, t531: f64, t2923: f64, t3785: f64, t9531: f64, t9448: f64, t9625: f64, t4544: f64, t11597: f64, t11605: f64, t11608: f64, t11611: f64, t26113: f64, t2842: f64, t5198: f64, t9458: f64, t9597: f64, t9598: f64, t9603: f64, t9604: f64, t9621: f64, t9737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31106 = t3663 * t13687;
    let t31114 = t3663 * t9566;
    let t31119 = t531 * t4524;
    let t31120 = t31119 * t2923;
    let t31123 = t3785 * t9531;
    let t31126 = t9625 * t9448;
    let t31136 = t531 * t4544 * t2923;
    let t31139 = -11200.0_f64 / 9.0_f64 * t9621 * t31106 + 800.0_f64 / 9.0_f64 * t5198 * t9597 * t9604 - 3200.0_f64 / 27.0_f64 * t9598 * t31106 + 3200.0_f64 / 27.0_f64 * t9598 * t31114 - 1600.0_f64 / 9.0_f64 * t9603 * t31106 - 2016.0_f64 * t26113 * t31120 + 24.0_f64 * t31123 * t11597 - 2016.0_f64 * t26113 * t31126 + 88.0_f64 / 27.0_f64 * t11605 * t2842 - 1600.0_f64 / 9.0_f64 * t11608 * t9458 + 8000.0_f64 / 9.0_f64 * t11611 * t9458 + 252.0_f64 * t9737 * t31136;
    (t31106, t31114, t31120, t31126, t31136, t31139)
}
