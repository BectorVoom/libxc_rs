//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1147/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1147<F: Float>(t1433: F, t1437: F, t72: F, t5445: F, t79: F, t5392: F, t605: F, t5399: F, t1860: F, t1865: F, t22544: F, t26013: F, t26016: F, t26051: F, t26084: F, t27937: F, t27950: F, t27953: F, t27957: F, t27961: F, t27966: F, t6490: F, t7428: F, t7432: F, t7435: F, t7442: F, t7446: F) -> (F, F, F, F, F) {
    let t27971 = t1433 * t1437;
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    let t27976 = t72 * t27975;
    let t27979 = t605 * t5392;
    let t27982 = t605 * t5399;
    let t27991 = -t27937 * t1865 / F::cast_from(6.0_f64) - t7428 * t7442 / F::cast_from(3.0_f64) - t7428 * t7446 / F::cast_from(3.0_f64) - t1860 * t27950 / F::cast_from(6.0_f64) - t1860 * t27953 / F::cast_from(3.0_f64) - t1860 * t27957 / F::cast_from(6.0_f64) - F::cast_from(5.0_f64) * t22544 * t27961 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t26013 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27966 * t1865 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26084 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t6490 * t27972 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6490 * t27976 + t27979 * t1865 / F::cast_from(3.0_f64) + t27982 * t1865 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t7442 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t7446 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26051 * t7432;
    (t27972, t27976, t27979, t27982, t27991)
}
