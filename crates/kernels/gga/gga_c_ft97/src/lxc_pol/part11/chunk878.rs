//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 878/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk878<F: Float>(t22: F, t37991: F, t25: F, t30: F, t6: F, t64: F, t8052: F, t11140: F, t11246: F, t11361: F, t1300: F, t1602: F, t1604: F, t1625: F, t1669: F, t1685: F, t1687: F, t1698: F, t1701: F, t1702: F, t1704: F, t1710: F, t1712: F, t1751: F, t2035: F, t22696: F, t22852: F, t3065: F, t3076: F, t372: F, t37473: F, t37614: F, t37641: F, t37960: F, t37968: F, t37971: F, t37978: F, t37985: F, t37987: F, t428: F, t7867: F, t7877: F, t7879: F, t7883: F, t7982: F, t8044: F, t8051: F, t8053: F, t8139: F, t8146: F, t8147: F, t8807: F) -> (F, F, F) {
    let t37993 = F::new(96.0) * t37991 * t22;
    let t37996 = t25 / t30 / t37993;
    let t38013 = t64 * t8052 * t6;
    let t38035 = -F::cast_from(0.1422571355482203117e0_f64) * t22852 * t1704 + F::cast_from(0.46477736175058559857e-2_f64) * t11246 * t37960 * t1604 - F::cast_from(0.23238868087529279928e-2_f64) * t11361 * t37960 * t1625 + F::cast_from(0.81118562704294997116e-3_f64) * t7982 * t37968 - F::cast_from(0.46477736175058559857e-2_f64) * t37971 * t11140 * t7879 + F::cast_from(0.16864243845320605903e-2_f64) * t1687 * t1698 + F::cast_from(0.279058811357253504e0_f64) * t7877 * t3065 * t37978 - F::cast_from(0.558117622714507008e0_f64) * t1602 * t37473 * t8044 - F::cast_from(0.47728159784266869676e-8_f64) * t37985 * t3065 * t37987 + F::cast_from(0.6139293849859577088e-2_f64) * t372 * t37996 * t37641 + F::cast_from(0.22524046461801549353e0_f64) * t1300 * t1701 * t7883 * t1751 + F::cast_from(0.10680687768703050405e-1_f64) * t7867 * t2035 * t8807 * t428 - F::cast_from(0.11011756047991868572e1_f64) * t1300 * t1701 * t37614 * t428 - F::cast_from(0.14225713554822031171e0_f64) * t38013 * t1701 * t1702 * t8053 + F::new(24.0) * t1669 * t1710 * t1685 * t1712 - F::new(36.0) * t3076 * t8051 * t1712 * t1751 - F::new(12.0) * t1669 * t8146 * t1751 + F::new(8.0) * t3076 * t1710 * t8139 * t428 - F::new(24.0) * t22696 * t8147;
    (t37993, t37996, t38035)
}
