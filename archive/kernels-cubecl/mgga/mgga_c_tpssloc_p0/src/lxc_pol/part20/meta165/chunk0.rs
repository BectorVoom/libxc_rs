//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1040/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1040<F: Float>(t1352: F, t3901: F, t1380: F, t3851: F, t3856: F, t3879: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3777: F, t3898: F, t544: F, t564: F) -> (F, F, F, F, F) {
    let t3902 = t3901 * t1352;
    let t3905 = t1380 * t3851;
    let t3907 = t1380 * t3856;
    let t3909 = t553 * t3879;
    let t3911 = F::cast_from(2.0_f64) * t1332 * t1383 + F::cast_from(2.0_f64) * t1336 * t3898 - F::cast_from(2.0_f64) * t1336 * t3902 - t1336 * t3905 - t1336 * t3907 - F::cast_from(2.0_f64) * t1381 * t3777 + t3773 * t564 + t3909 * t544;
    (t3902, t3905, t3907, t3909, t3911)
}
