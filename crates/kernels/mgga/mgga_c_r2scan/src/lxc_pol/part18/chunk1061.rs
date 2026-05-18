//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1061/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1061<F: Float>(t2096: F, t265: F, t267: F, t37625: F, t546: F, t6476: F, t565: F, t6481: F, t2111: F, t409: F, t5148: F, t2157: F, t625: F) -> (F, F, F, F) {
    let t37628 = t2096 * t265 * t267;
    let t37630 = t546 * t37625 * t37628 * t6476;
    let t37634 = t565 * t37625 * t37628 * t6481;
    let t37637 = t2111 * t409 * t5148;
    let t37638 = t2157 * t625;
    (t37630, t37634, t37637, t37638)
}
