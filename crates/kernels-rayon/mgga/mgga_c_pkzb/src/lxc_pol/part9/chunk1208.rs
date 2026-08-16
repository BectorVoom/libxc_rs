//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1208/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1208(t17536: f64, t7286: f64, t1855: f64, t683: f64, t7444: f64, t1893: f64, t2783: f64, t1856: f64, t5776: f64, t7278: f64, t1084: f64, t5796: f64) -> (f64, f64, f64, f64, f64) {
    let t21018 = 0.1551780387578202009e4_f64 * t17536 * t7286;
    let t21021 = 6.0_f64 * t1855 * t7444 * t683;
    let t21024 = 6.0_f64 * t1855 * t2783 * t1893;
    let t21027 = 0.28947563097646563121e3_f64 * t5776 * t7278 * t1856;
    let t21030 = 2.0_f64 * t1855 * t1084 * t5796;
    (t21018, t21021, t21024, t21027, t21030)
}
