//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 987/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk987(t120605: f64, t120610: f64, t120197: f64, t1799: f64, t22633: f64, t22635: f64, t1842: f64, t31090: f64, t28209: f64, t31137: f64, t6888: f64, t1985: f64, t26193: f64, t32697: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127422 = 0.15352717957250113407e0_f64 * t120605;
    let t127423 = 0.76763589786250567036e-1_f64 * t120610;
    let t127427 = 0.6579736267392905746e-1_f64 * t22633 * t22635 * t120197 * t1799;
    let t127430 = t1799 * t1842;
    let t127434 = 0.13159472534785811492e0_f64 * t22633 * t22635 * t31090 * t127430;
    let t127442 = 0.3289868133696452873e-1_f64 * t6888 * t31137 * t28209;
    let t127445 = 0.3289868133696452873e-1_f64 * t1985 * t26193 * t32697;
    (t127422, t127423, t127427, t127430, t127434, t127442, t127445)
}
