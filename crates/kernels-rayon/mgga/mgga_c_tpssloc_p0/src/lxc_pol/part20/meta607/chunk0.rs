//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2191/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191(t3503: f64, t44833: f64, t44834: f64, t1174: f64, t1197: f64, t2402: f64, t3584: f64, t676: f64, t1227: f64, t248: f64, t3243: f64, t1011: f64, t1212: f64, t44706: f64) -> (f64, f64, f64, f64) {
    let t45037 = t44833 * t3503 * t44834;
    let t45044 = t1174 * t2402 * t1197;
    let t45046 = t676 * t3584;
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45080 = t44706 * t1011 * t1212;
    (t45037, t45044, t45049, t45080)
}
