//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1840/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840(t12571: f64, t23966: f64, t6492: f64, t7432: f64, t84195: f64, t23967: f64, t26067: f64, t23993: f64, t7428: f64, t23998: f64, t1860: f64, t23992: f64, t7445: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91957 = t12571 * t23966;
    let t91959 = 80.0_f64 / 9.0_f64 * t91957 * t6492;
    let t91961 = 80.0_f64 / 9.0_f64 * t84195 * t7432;
    let t91980 = 80.0_f64 / 9.0_f64 * t23967 * t26067;
    let t91996 = t7428 * t23993;
    let t92001 = 16.0_f64 / 9.0_f64 * t7428 * t23998;
    let t92003 = t1860 * t23992 * t7445;
    (t91957, t91959, t91961, t91980, t91996, t92001, t92003)
}
