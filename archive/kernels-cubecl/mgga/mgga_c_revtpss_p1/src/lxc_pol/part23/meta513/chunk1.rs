//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2015/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2015<F: Float>(t1042: F, t21094: F, t1038: F, t6593: F, t1244: F, t1241: F) -> (F, F, F) {
    let t21095 = t1042 * t21094;
    let t21100 = t6593 * t1038;
    let t21101 = t1244 * t21100;
    let t21102 = t1241 * t21101;
    (t21095, t21101, t21102)
}
