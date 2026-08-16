//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1022/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1022(t17800: f64, t4514: f64, t17794: f64, t4531: f64, t10339: f64, t13896: f64, t17764: f64, t17770: f64, t17827: f64, t17850: f64, t21410: f64, t21413: f64, t21416: f64, t2986: f64, t973: f64) -> f64 {
    let t21419 = t17800 * t4514;
    let t21422 = t4531 * t17794;
    let t21429 = -0.83333333333333333331e-3_f64 * t17827 - 0.22222222222222222221e-2_f64 * t973 * t21410 + 0.11111111111111111111e-2_f64 * t2986 * t21413 - 0.11111111111111111111e-2_f64 * t2986 * t21416 - 0.83333333333333333331e-3_f64 * t2986 * t21419 - 0.83333333333333333331e-3_f64 * t2986 * t21422 - 0.55555555555555555554e-3_f64 * t17764 + 0.27777777777777777777e-3_f64 * t17770 - 0.83333333333333333331e-3_f64 * t17850 + t10339 - 0.18518518518518518518e-3_f64 * t13896;
    t21429
}
