//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 750/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk750(t5399: f64, t605: f64, t1860: f64, t1865: f64, t22544: f64, t26013: f64, t26016: f64, t26051: f64, t26084: f64, t27937: f64, t27950: f64, t27953: f64, t27957: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t27979: f64, t6490: f64, t7428: f64, t7432: f64, t7435: f64, t7442: f64, t7446: f64) -> (f64, f64) {
    let t27982 = t605 * t5399;
    let t27991 = -t27937 * t1865 / 6.0_f64 - t7428 * t7442 / 3.0_f64 - t7428 * t7446 / 3.0_f64 - t1860 * t27950 / 6.0_f64 - t1860 * t27953 / 3.0_f64 - t1860 * t27957 / 6.0_f64 - 5.0_f64 * t22544 * t27961 - 10.0_f64 / 3.0_f64 * t26016 * t26013 + 2.0_f64 / 3.0_f64 * t27966 * t1865 + 5.0_f64 / 3.0_f64 * t26084 * t7432 + 5.0_f64 / 3.0_f64 * t6490 * t27972 + 5.0_f64 / 6.0_f64 * t6490 * t27976 + t27979 * t1865 / 3.0_f64 + t27982 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7435 * t7442 + 2.0_f64 / 3.0_f64 * t7435 * t7446 + 5.0_f64 / 3.0_f64 * t26051 * t7432;
    (t27982, t27991)
}
