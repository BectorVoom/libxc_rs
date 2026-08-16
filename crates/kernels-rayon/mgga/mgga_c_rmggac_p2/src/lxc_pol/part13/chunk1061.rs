//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1061/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1061(t39785: f64, t39796: f64, t39800: f64, t39808: f64, t39752: f64, t39754: f64, t39756: f64, t39758: f64, t39760: f64, t39764: f64, t39771: f64, t39773: f64, t39777: f64, t39781: f64, t39789: f64, t39792: f64, t39804: f64, t5019: f64, t9402: f64) -> f64 {
    let t43135 = 0.60975299583150056624e-3_f64 * t39785;
    let t43138 = 0.60975299583150056624e-3_f64 * t39796;
    let t43139 = 0.60975299583150056624e-3_f64 * t39800;
    let t43141 = 0.86737941314158990616e-4_f64 * t39808;
    let t43144 = -0.15323255961587222184e-3_f64 * t39752 - 0.1702583995731913576e-4_f64 * t39754 - 0.1064114997332445985e-4_f64 * t39756 - 0.212822999466489197e-4_f64 * t39758 + 0.5107751987195740728e-4_f64 * t39760 + 0.5107751987195740728e-4_f64 * t39764 + 0.2553875993597870364e-4_f64 * t39771 + 0.3405167991463827152e-4_f64 * t39773 + 0.3405167991463827152e-4_f64 * t39777 + 0.1702583995731913576e-4_f64 * t39781 - t43135 - 0.30487649791575028312e-3_f64 * t39789 - 0.39032073591371545778e-3_f64 * t39792 - t43138 - t43139 - 0.30487649791575028312e-3_f64 * t39804 + t43141 - 0.47896966807455234256e0_f64 * t5019 * t9402;
    t43144
}
