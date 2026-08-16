//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2115/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2115(t1904: f64, t5599: f64, t689: f64, t10157: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t14276: f64, t5715: f64, t5728: f64, t9694: f64, t9695: f64) -> (f64, f64, f64) {
    let t22427 = t5599 * t1904;
    let t22428 = t689 * t22427;
    let t22430 = t9694 + 0.26019841438354088051e-1_f64 * t14091 - 0.13009920719177044025e-1_f64 * t9695 + 0.26341796731742046394e1_f64 * t5715 * t5728 + t14096 + 0.14634331517634470219e-1_f64 * t14097 - t14102 - 0.23131639038696784278e-2_f64 * t14105 - t14108 + 0.39029762157531132076e-1_f64 * t14111 + 0.10975748638225852664e-1_f64 * t22428 + t14276 - t10157;
    (t22427, t22428, t22430)
}
