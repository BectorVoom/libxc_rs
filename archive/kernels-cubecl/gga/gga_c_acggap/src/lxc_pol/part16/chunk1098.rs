//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1098/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1098<F: Float>(t1181: F, t5876: F, t604: F, t7575: F, t1839: F, t1992: F, t7585: F, t7842: F, t1089: F, t1894: F, t2079: F, t2080: F) -> (F, F, F) {
    let t39209 = t7575 * t1181 * t604 * t5876;
    let t39213 = t7585 * t7842 * t1992 * t1839;
    let t39217 = t2079 * t1089 * t1894 * t2080;
    (t39209, t39213, t39217)
}
