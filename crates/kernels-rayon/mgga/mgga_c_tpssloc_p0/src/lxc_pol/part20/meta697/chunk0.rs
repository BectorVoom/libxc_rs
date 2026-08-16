//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2661/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2661(t1788: f64, t9212: f64, t9214: f64, t2223: f64, t5168: f64, t39328: f64, t39330: f64, t39334: f64, t39339: f64, t39341: f64, t15977: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54312 = t9212 * t1788;
    let t54313 = 24.0_f64 * t54312;
    let t54314 = t9214 * t1788;
    let t54315 = 144.0_f64 * t54314;
    let t54316 = t2223 * t5168;
    let t54317 = 96.0_f64 * t54316;
    let t54318 = 0.48796115851357829289e-1_f64 * t39328;
    let t54319 = 0.73245789224026180215e-3_f64 * t39330;
    let t54320 = 0.18311447306006545054e-3_f64 * t39334;
    let t54321 = 0.10526802520742363173e2_f64 * t39339;
    let t54322 = 0.15584273195113317383e3_f64 * t39341;
    let t54323 = t588 * t15977;
    (t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54323)
}
