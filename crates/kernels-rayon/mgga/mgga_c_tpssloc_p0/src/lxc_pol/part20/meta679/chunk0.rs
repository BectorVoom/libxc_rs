//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2563/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563(t11126: f64, t4875: f64, t14858: f64, t3415: f64, t11294: f64, t4869: f64, t15044: f64, t3411: f64, t11300: f64, t1164: f64, t14841: f64, t3419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51853 = 0.35089341735807877242e1_f64 * t11126 * t4875;
    let t51855 = 0.35089341735807877242e1_f64 * t14858 * t3415;
    let t51857 = 0.10389515463408878255e3_f64 * t4869 * t11294;
    let t51859 = 0.35089341735807877242e1_f64 * t3411 * t15044;
    let t51862 = 0.14035736694323150897e2_f64 * t1164 * t14841 * t11300;
    let t51864 = 0.17544670867903938621e1_f64 * t14858 * t3419;
    (t51853, t51855, t51857, t51859, t51862, t51864)
}
