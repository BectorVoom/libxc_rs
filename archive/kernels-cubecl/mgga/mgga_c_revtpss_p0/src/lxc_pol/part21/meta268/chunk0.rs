//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1483/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1483<F: Float>(t1390: F, t828: F, t9899: F, t221: F, t4019: F, t4057: F, t4018: F, t1386: F, t2681: F, t820: F) -> (F, F, F, F) {
    let t9901 = t1390 * t828 * t9899;
    let t9905 = t4019 * t221 * t4057;
    let t9906 = t4018 * t9905;
    let t9909 = t820 * t1386 * t2681;
    (t9901, t9905, t9906, t9909)
}
