//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2656/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2656<F: Float>(t19127: F, t2926: F, t2873: F, t6104: F, t11108: F, t6396: F, t19226: F, t2970: F, t2986: F, t6184: F, t11509: F, t6205: F) -> (F, F, F, F, F, F) {
    let t63650 = t19127 * t2926;
    let t63677 = t6104 * t2873;
    let t63907 = t6396 * t11108;
    let t63971 = t19226 * t2970;
    let t63997 = t6184 * t2986;
    let t64043 = t6205 * t11509;
    (t63650, t63677, t63907, t63971, t63997, t64043)
}
