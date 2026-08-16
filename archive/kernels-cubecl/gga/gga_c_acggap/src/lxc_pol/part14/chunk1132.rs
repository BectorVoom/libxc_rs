//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1132/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1132<F: Float>(t1181: F, t5969: F, t599: F, t7493: F, t1839: F, t1983: F, t7585: F, t7586: F, t7839: F, t9641: F, t1165: F, t2068: F, t604: F, t6069: F) -> (F, F, F, F) {
    let t39669 = t7493 * t1181 * t599 * t5969;
    let t39673 = t7585 * t7586 * t1983 * t1839;
    let t39675 = t7839 * t9641;
    let t39679 = t2068 * t1165 * t604 * t6069;
    (t39669, t39673, t39675, t39679)
}
