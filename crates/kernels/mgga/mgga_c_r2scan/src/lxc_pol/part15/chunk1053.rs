//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1053/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1053<F: Float>(t11640: F, t24039: F, t11643: F, t22731: F, t11654: F, t6395: F, t10869: F, t7601: F, t10811: F, t2651: F, t10903: F, t11764: F, t2207: F, t2147: F, t26307: F, t3332: F) -> (F, F, F, F, F, F, F) {
    let t40149 = t24039 * t11640;
    let t40151 = t22731 * t11643;
    let t40153 = t6395 * t11654;
    let t40155 = t7601 * t10869;
    let t40156 = 0.46574606203128791246e-1 * t40155;
    let t40157 = t2651 * t10811;
    let t40158 = 0.23115257973478049502e0 * t40157;
    let t40162 = t2207 * t10903 * t11764;
    let t40165 = t2147 * t3332 * t26307;
    (t40149, t40151, t40153, t40156, t40158, t40162, t40165)
}
