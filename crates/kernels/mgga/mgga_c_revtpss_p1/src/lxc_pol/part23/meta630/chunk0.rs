//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2324/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2324<F: Float>(t159: F, t2698: F, t1518: F, t648: F, t4292: F, t94: F, t1353: F, t1907: F, t1583: F, t775: F, t890: F, t1014: F, t65: F) -> (F, F, F, F, F, F, F) {
    let t25273 = t2698 * t159;
    let t27123 = t648 * t1518;
    let t27126 = t94 * t4292;
    let t27153 = t1907 * t1353;
    let t27375 = t1583 * t775;
    let t27384 = t1583 * t890;
    let t27527 = t65 * t1014;
    (t25273, t27123, t27126, t27153, t27375, t27384, t27527)
}
