//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1177/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1177(t12222: f64, t16081: f64, t116: f64, t1314: f64, t9534: f64, t1307: f64, t133: f64, t6600: f64, t12226: f64, t16094: f64, t3719: f64, t686: f64) -> (f64, f64, f64) {
    let t40366 = t16081 * t12222;
    let t40369 = t9534 * t1314 * t116;
    let t40372 = t40369 * t133 * t6600 * t1307;
    let t40376 = t16094 * t686 * t12226 * t3719;
    (t40366, t40372, t40376)
}
