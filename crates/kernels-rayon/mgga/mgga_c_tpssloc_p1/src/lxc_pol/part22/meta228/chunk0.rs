//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1289/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289(t2588: f64, t9577: f64, t21: f64, t59: f64, t207: f64, t795: f64, t2690: f64, t841: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t9579 = 0.99999999999999999997e-2_f64 * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = 0.16435185185185185185e-1_f64 * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    (t9579, t9580, t9583, t9600, t9601)
}
