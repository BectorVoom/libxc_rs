//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1029/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1029<F: Float>(t4872: F, t4873: F, t1042: F, t1032: F, t1647: F, t1040: F) -> (F, F, F, F) {
    let t4874 = t4872 * t4873;
    let t4875 = t1042 * t4874;
    let t4878 = t1647 * t1032;
    let t4879 = t4878 * t1040;
    (t4874, t4875, t4878, t4879)
}
