//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1003/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1003(t1268: f64, t2312: f64, t2314: f64, t2319: f64, t2363: f64, t671: f64, t88: f64, t526: f64) -> (f64, f64) {
    let t3660 = 2.0_f64 * t1268 * t2363 + 4.0_f64 * t2314 * t671 + 2.0_f64 * t2319 * t88 + t2312;
    let t3664 = 1.0_f64 / t526;
    (t3660, t3664)
}
