//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1110/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1110<F: Float>(t10869: F, t7601: F, t10811: F, t2651: F, t10903: F, t11764: F, t2207: F, t261: F, t3299: F, t7390: F, t10879: F, t11727: F) -> (F, F, F, F, F) {
    let t40155 = t7601 * t10869;
    let t40156 = F::new(0.46574606203128791246e-1) * t40155;
    let t40157 = t2651 * t10811;
    let t40158 = F::new(0.23115257973478049502e0) * t40157;
    let t40162 = t2207 * t10903 * t11764;
    let t40175 = t3299 * t261 * t7390;
    let t40176 = F::new(0.23115257973478049502e0) * t40175;
    let t40177 = t10879 * t11727;
    (t40156, t40158, t40162, t40176, t40177)
}
