//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 842/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk842<F: Float>(t6038: F, t759: F, t4733: F, t4736: F, t4739: F, t5860: F) -> (F, F) {
    let t6039 = t759 * t6038;
    let t6044 = -0.29633333333333333333e-1 * t4733 + 0.19755555555555555555e-1 * t4736 - 0.23048148148148148148e-1 * t4739 - t5860;
    (t6039, t6044)
}
