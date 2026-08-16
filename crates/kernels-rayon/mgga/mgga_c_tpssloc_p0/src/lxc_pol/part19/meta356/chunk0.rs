//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1287/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1287(t2793: f64, t10661: f64, t913: f64, t2836: f64, t2792: f64, t2842: f64, t2844: f64, t2880: f64, t2897: f64, t2904: f64, t10701: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41995 = t2793 * t2793;
    let t41998 = 24.0_f64 * t10661 * t41995 * t913;
    let t41999 = t2836 * t2836;
    let t42002 = 6.0_f64 * t2792 * t41999 * t913;
    let t42005 = 0.48245938496077605201e2_f64 * t2842 * t41999 * t2844;
    let t42011 = t2880 * t2880;
    let t42020 = t2897 * t2904;
    let t42023 = t888 * t10701;
    (t41995, t41998, t42002, t42005, t42011, t42020, t42023)
}
