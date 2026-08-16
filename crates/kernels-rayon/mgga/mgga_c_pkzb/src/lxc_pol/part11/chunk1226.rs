//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1226/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1226(t2782: f64, t3554: f64, t5776: f64, t10868: f64, t17541: f64, t683: f64, t10919: f64, t1855: f64, t10918: f64, t1901: f64, t1899: f64, t5802: f64, t9235: f64) -> (f64, f64, f64, f64, f64) {
    let t30242 = 0.28947563097646563121e3_f64 * t5776 * t3554 * t2782;
    let t30245 = 0.62071215503128080361e4_f64 * t17541 * t10868 * t683;
    let t30248 = 2.0_f64 * t1855 * t10919 * t683;
    let t30249 = t10918 * t1901;
    let t30252 = 0.16081979498692535067e2_f64 * t1899 * t30249 * t683;
    let t30255 = 0.1551780387578202009e4_f64 * t5802 * t9235 * t2782;
    (t30242, t30245, t30248, t30252, t30255)
}
