//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1441/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441(t11126: f64, t3419: f64, t11478: f64, t3411: f64, t3633: f64, t3415: f64, t1164: f64, t3400: f64, t3403: f64, t44168: f64, t1156: f64, t3375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44375 = 0.35089341735807877242e1_f64 * t11126 * t3419;
    let t44377 = 0.23392894490538584828e1_f64 * t3411 * t11478;
    let t44378 = t3633 * t3633;
    let t44384 = 0.70178683471615754484e1_f64 * t11126 * t3415;
    let t44388 = 0.51947577317044391277e2_f64 * t1164 * t3400 * t44168 * t3403;
    let t44392 = 0.35089341735807877242e1_f64 * t1164 * t3375 * t44168 * t1156;
    (t44375, t44377, t44378, t44384, t44388, t44392)
}
