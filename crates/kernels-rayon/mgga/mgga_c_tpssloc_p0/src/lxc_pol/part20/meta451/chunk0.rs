//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1901/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1901(t15067: f64, t3265: f64, t11275: f64, t14704: f64, t14710: f64, t14720: f64, t11215: f64, t11217: f64, t14722: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64) -> (f64, f64, f64, f64, f64) {
    let t15068 = t15067 * t3265;
    let t15070 = 0.51726012919273400301e3_f64 * t11275 * t15068;
    let t15072 = 0.34431666666666666666e0_f64 * t14704;
    let t15074 = 0.13892666666666666667e0_f64 * t14710;
    let t15083 = 0.22954444444444444444e0_f64 * t14720;
    let t15091 = -0.13892666666666666667e0_f64 * t11215 - 0.69463333333333333333e-1_f64 * t11217 + 0.11577222222222222222e0_f64 * t14766 + t15083 - 0.68863333333333333334e0_f64 * t14738 - 0.34431666666666666667e0_f64 * t14742 - 0.20659e1_f64 * t14733 + 0.20659e1_f64 * t14751 + 0.103295e1_f64 * t14755 + 0.309885e1_f64 * t14746 - 0.68863333333333333333e0_f64 * t14722;
    (t15068, t15070, t15072, t15074, t15091)
}
