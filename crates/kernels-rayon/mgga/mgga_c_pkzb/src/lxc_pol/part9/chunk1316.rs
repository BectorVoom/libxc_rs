//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1316/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1316(t3214: f64, t6531: f64, t6488: f64, t8368: f64, t2380: f64, t2383: f64, t3236: f64, t54: f64, t6491: f64, t8360: f64, t1238: f64, t6400: f64) -> (f64, f64, f64, f64, f64) {
    let t23248 = t3214 * t6531;
    let t23250 = t8368 * t6488;
    let t23254 = t2380 * t54 * t3236 * t2383;
    let t23264 = t8360 * t6491;
    let t23266 = t1238 * t6400;
    (t23248, t23250, t23254, t23264, t23266)
}
