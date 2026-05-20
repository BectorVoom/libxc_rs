//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2720/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2720<F: Float>(t1904: F, t5599: F, t689: F, t10157: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t14276: F, t5715: F, t5728: F, t9694: F, t9695: F) -> (F, F) {
    let t22427 = t5599 * t1904;
    let t22428 = t689 * t22427;
    let t22430 = t9694 + F::cast_from(0.26019841438354088051e-1_f64) * t14091 - F::cast_from(0.13009920719177044025e-1_f64) * t9695 + F::cast_from(0.26341796731742046394e1_f64) * t5715 * t5728 + t14096 + F::cast_from(0.14634331517634470219e-1_f64) * t14097 - t14102 - F::cast_from(0.23131639038696784278e-2_f64) * t14105 - t14108 + F::cast_from(0.39029762157531132076e-1_f64) * t14111 + F::cast_from(0.10975748638225852664e-1_f64) * t22428 + t14276 - t10157;
    (t22427, t22430)
}
