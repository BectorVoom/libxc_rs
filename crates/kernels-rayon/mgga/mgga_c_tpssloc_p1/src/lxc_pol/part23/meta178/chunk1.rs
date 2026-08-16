//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 805/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk805(t2374: f64, t9885: f64, t2528: f64, t677: f64, t2509: f64, t745: f64, t9843: f64) -> (f64, f64, f64, f64) {
    let t9887 = 0.16265371950452609763e-1_f64 * t2374 * t9885;
    let t9888 = t677 * t2528;
    let t9890 = 0.48159733137676571078e0_f64 * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    (t9887, t9888, t9890, t9892)
}
