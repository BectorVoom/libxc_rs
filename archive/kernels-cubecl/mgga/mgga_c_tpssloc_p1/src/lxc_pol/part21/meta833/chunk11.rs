//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2952/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952<F: Float>(t2986: F, t4514: F, t48019: F, t48046: F, t10186: F, t10259: F, t17742: F, t17745: F, t17749: F, t17794: F, t17801: F, t17817: F, t25608: F, t3014: F, t343: F, t4510: F, t4518: F, t4531: F, t4546: F, t5836: F, t59719: F, t59746: F, t884: F, t973: F) -> F {
    let t61489 = t2986 * t48019 * t4514;
    let t61495 = t2986 * t48046 * t4514;
    let t61523 = F::cast_from(0.12345679012345679012e-3_f64) * t61489 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4518 * t59746 - F::cast_from(0.37037037037037037036e-3_f64) * t61495 + F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t10259 * t17817 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t10259 * t17794 + F::cast_from(0.14814814814814814814e-2_f64) * t10186 * t17801 + F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t4510 * t59719 - F::cast_from(0.59259259259259259257e-2_f64) * t10186 * t17742 + F::cast_from(0.39506172839506172838e-2_f64) * t10186 * t17745 + F::cast_from(0.29629629629629629628e-2_f64) * t10186 * t17749 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4531 * t25608 * t884 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t5836 * t3014 * t343;
    t61523
}
