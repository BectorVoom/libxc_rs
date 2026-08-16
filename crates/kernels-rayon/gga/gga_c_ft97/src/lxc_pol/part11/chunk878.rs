//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 878/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk878(t22: f64, t37991: f64, t25: f64, t30: f64, t6: f64, t64: f64, t8052: f64, t11140: f64, t11246: f64, t11361: f64, t1300: f64, t1602: f64, t1604: f64, t1625: f64, t1669: f64, t1685: f64, t1687: f64, t1698: f64, t1701: f64, t1702: f64, t1704: f64, t1710: f64, t1712: f64, t1751: f64, t2035: f64, t22696: f64, t22852: f64, t3065: f64, t3076: f64, t372: f64, t37473: f64, t37614: f64, t37641: f64, t37960: f64, t37968: f64, t37971: f64, t37978: f64, t37985: f64, t37987: f64, t428: f64, t7867: f64, t7877: f64, t7879: f64, t7883: f64, t7982: f64, t8044: f64, t8051: f64, t8053: f64, t8139: f64, t8146: f64, t8147: f64, t8807: f64) -> (f64, f64, f64) {
    let t37993 = 96.0_f64 * t37991 * t22;
    let t37996 = t25 / t30 / t37993;
    let t38013 = t64 * t8052 * t6;
    let t38035 = -0.1422571355482203117e0_f64 * t22852 * t1704 + 0.46477736175058559857e-2_f64 * t11246 * t37960 * t1604 - 0.23238868087529279928e-2_f64 * t11361 * t37960 * t1625 + 0.81118562704294997116e-3_f64 * t7982 * t37968 - 0.46477736175058559857e-2_f64 * t37971 * t11140 * t7879 + 0.16864243845320605903e-2_f64 * t1687 * t1698 + 0.279058811357253504e0_f64 * t7877 * t3065 * t37978 - 0.558117622714507008e0_f64 * t1602 * t37473 * t8044 - 0.47728159784266869676e-8_f64 * t37985 * t3065 * t37987 + 0.6139293849859577088e-2_f64 * t372 * t37996 * t37641 + 0.22524046461801549353e0_f64 * t1300 * t1701 * t7883 * t1751 + 0.10680687768703050405e-1_f64 * t7867 * t2035 * t8807 * t428 - 0.11011756047991868572e1_f64 * t1300 * t1701 * t37614 * t428 - 0.14225713554822031171e0_f64 * t38013 * t1701 * t1702 * t8053 + 24.0_f64 * t1669 * t1710 * t1685 * t1712 - 36.0_f64 * t3076 * t8051 * t1712 * t1751 - 12.0_f64 * t1669 * t8146 * t1751 + 8.0_f64 * t3076 * t1710 * t8139 * t428 - 24.0_f64 * t22696 * t8147;
    (t37993, t37996, t38035)
}
