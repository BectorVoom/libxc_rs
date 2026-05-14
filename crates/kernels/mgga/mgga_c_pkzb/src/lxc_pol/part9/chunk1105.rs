//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1105/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1105<F: Float>(t17326: F, t2751: F, t5734: F, t7269: F, t7272: F, t17329: F, t7275: F, t17655: F, t2787: F, t5771: F, t7279: F, t7282: F, t17536: F, t7286: F, t1855: F, t683: F, t7444: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21004 = 6.0 * t17326 * t2751;
    let t21006 = 12.0 * t5734 * t7269;
    let t21008 = 6.0 * t5734 * t7272;
    let t21010 = 0.28947563097646563121e3 * t17329 * t7275;
    let t21012 = 0.48245938496077605201e2 * t17655 * t2787;
    let t21014 = 0.96491876992155210402e2 * t5771 * t7279;
    let t21016 = 0.48245938496077605201e2 * t5771 * t7282;
    let t21018 = 0.1551780387578202009e4 * t17536 * t7286;
    let t21021 = 6.0 * t1855 * t7444 * t683;
    (t21004, t21006, t21008, t21010, t21012, t21014, t21016, t21018, t21021)
}
