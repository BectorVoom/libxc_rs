//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 875/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk875(t3565: f64, t702: f64, t1096: f64, t2815: f64, t3581: f64, t3578: f64, t1940: f64, t3577: f64, t2819: f64, t3564: f64, t5873: f64, t3592: f64, t721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9410 = t3565 * t702;
    let t9413 = t1096 * t2815;
    let t9416 = t3581 * t702;
    let t9419 = t3578 * t702;
    let t9422 = t3577 * t1940;
    let t9423 = t9422 * t702;
    let t9426 = t2819 * t2815;
    let t9429 = t3564 * t5873;
    let t9430 = t9429 * t702;
    let t9437 = t3592 * t721;
    (t9410, t9413, t9416, t9419, t9422, t9423, t9426, t9429, t9430, t9437)
}
