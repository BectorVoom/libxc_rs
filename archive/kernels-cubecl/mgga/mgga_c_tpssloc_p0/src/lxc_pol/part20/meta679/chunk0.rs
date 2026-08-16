//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2563/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563<F: Float>(t11126: F, t4875: F, t14858: F, t3415: F, t11294: F, t4869: F, t15044: F, t3411: F, t11300: F, t1164: F, t14841: F, t3419: F) -> (F, F, F, F, F, F) {
    let t51853 = F::cast_from(0.35089341735807877242e1_f64) * t11126 * t4875;
    let t51855 = F::cast_from(0.35089341735807877242e1_f64) * t14858 * t3415;
    let t51857 = F::cast_from(0.10389515463408878255e3_f64) * t4869 * t11294;
    let t51859 = F::cast_from(0.35089341735807877242e1_f64) * t3411 * t15044;
    let t51862 = F::cast_from(0.14035736694323150897e2_f64) * t1164 * t14841 * t11300;
    let t51864 = F::cast_from(0.17544670867903938621e1_f64) * t14858 * t3419;
    (t51853, t51855, t51857, t51859, t51862, t51864)
}
