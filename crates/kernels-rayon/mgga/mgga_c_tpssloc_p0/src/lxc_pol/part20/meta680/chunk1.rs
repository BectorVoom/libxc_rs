//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2566/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2566(t1164: f64, t14829: f64, t3400: f64, t4883: f64, t14960: f64, t3396: f64, t15036: f64, t3411: f64, t11126: f64, t4879: f64, t11634: f64, t4869: f64) -> (f64, f64, f64, f64, f64) {
    let t51889 = 0.51947577317044391277e2_f64 * t1164 * t3400 * t14829 * t4883;
    let t51892 = 0.35089341735807877242e1_f64 * t1164 * t14960 * t3396;
    let t51898 = 0.10526802520742363173e2_f64 * t3411 * t15036;
    let t51903 = 0.17544670867903938621e1_f64 * t11126 * t4879;
    let t51905 = 0.51947577317044391277e2_f64 * t4869 * t11634;
    (t51889, t51892, t51898, t51903, t51905)
}
