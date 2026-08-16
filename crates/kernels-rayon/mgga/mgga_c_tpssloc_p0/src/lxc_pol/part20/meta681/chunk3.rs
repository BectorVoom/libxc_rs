//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2570/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570(t14753: f64, t15402: f64, t3447: f64, t11509: f64, t11566: f64, t11576: f64, t11580: f64, t11585: f64, t11594: f64, t15376: f64, t15395: f64, t3449: f64, t4900: f64, t4908: f64, t50879: f64, t50884: f64, t50915: f64, t50929: f64, t51948: f64, t51961: f64, t51971: f64, t51975: f64, t51981: f64, t51988: f64) -> f64 {
    let t51991 = t3447 * t15402 * t14753;
    let t51993 = 0.22222222222222222222e-2_f64 * t51948 - 0.22222222222222222222e-2_f64 * t15376 * t11594 - 0.22222222222222222222e-2_f64 * t15376 * t11576 - 0.22222222222222222222e-2_f64 * t15376 * t11580 - 0.44444444444444444445e-2_f64 * t15376 * t11585 + 0.11111111111111111111e-2_f64 * t3447 * t4900 * t50884 + 0.83333333333333333331e-3_f64 * t3447 * t3449 * t51961 - 0.16666666666666666666e-2_f64 * t3447 * t4908 * t50929 - t51971 - 0.66666666666666666664e-2_f64 * t3447 * t4908 * t50879 - 0.24999999999999999999e-2_f64 * t3447 * t51975 * t11509 - t51981 - 0.1037037037037037037e-1_f64 * t3447 * t15395 * t50915 - 0.29629629629629629629e-2_f64 * t15376 * t11566 + 0.27777777777777777777e-3_f64 * t51988 - 0.55555555555555555554e-3_f64 * t51991;
    t51993
}
