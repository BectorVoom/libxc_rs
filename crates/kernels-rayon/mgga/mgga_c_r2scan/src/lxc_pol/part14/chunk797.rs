//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 797/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk797(t5021: f64, t5872: f64, t5874: f64, t5871: f64, t5878: f64, t1509: f64, t898: f64, t41: f64, t1531: f64, t2463: f64, t2: f64, t2483: f64) -> (f64, f64, f64, f64, f64) {
    let t7025 = 4.0_f64 * t5021;
    let t7026 = 1584.0_f64 * t5872;
    let t7027 = 1872.0_f64 * t5874;
    let t7028 = t5871 - t7026 - t7027 + t5878;
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7033 = 0.24415263074675393405e-3_f64 * t7032;
    let t7034 = t2483 * t2;
    (t7025, t7028, t7031, t7033, t7034)
}
