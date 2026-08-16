//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2080/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2080(t91310: f64, t26245: f64, t80791: f64, t26271: f64, t80836: f64, t1361: f64, t22690: f64, t22792: f64, t5187: f64, t1307: f64, t7708: f64, t80840: f64, t90787: f64) -> (f64, f64, f64, f64, f64) {
    let t91311 = 0.6728792682356731809e-4_f64 * t91310;
    let t91312 = t80791 * t26245;
    let t91323 = t80836 * t26271;
    let t91327 = t22792 * t22690 * t1361 * t5187;
    let t91328 = 0.40372756094140390854e-3_f64 * t91327;
    let t91344 = t80840 * t90787 * t7708 * t1307;
    (t91311, t91312, t91323, t91328, t91344)
}
