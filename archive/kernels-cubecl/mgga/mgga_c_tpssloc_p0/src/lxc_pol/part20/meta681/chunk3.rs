//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2570/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2570<F: Float>(t14753: F, t15402: F, t3447: F, t11509: F, t11566: F, t11576: F, t11580: F, t11585: F, t11594: F, t15376: F, t15395: F, t3449: F, t4900: F, t4908: F, t50879: F, t50884: F, t50915: F, t50929: F, t51948: F, t51961: F, t51971: F, t51975: F, t51981: F, t51988: F) -> F {
    let t51991 = t3447 * t15402 * t14753;
    let t51993 = F::cast_from(0.22222222222222222222e-2_f64) * t51948 - F::cast_from(0.22222222222222222222e-2_f64) * t15376 * t11594 - F::cast_from(0.22222222222222222222e-2_f64) * t15376 * t11576 - F::cast_from(0.22222222222222222222e-2_f64) * t15376 * t11580 - F::cast_from(0.44444444444444444445e-2_f64) * t15376 * t11585 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4900 * t50884 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t3449 * t51961 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t50929 - t51971 - F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t4908 * t50879 - F::cast_from(0.24999999999999999999e-2_f64) * t3447 * t51975 * t11509 - t51981 - F::cast_from(0.1037037037037037037e-1_f64) * t3447 * t15395 * t50915 - F::cast_from(0.29629629629629629629e-2_f64) * t15376 * t11566 + F::cast_from(0.27777777777777777777e-3_f64) * t51988 - F::cast_from(0.55555555555555555554e-3_f64) * t51991;
    t51993
}
