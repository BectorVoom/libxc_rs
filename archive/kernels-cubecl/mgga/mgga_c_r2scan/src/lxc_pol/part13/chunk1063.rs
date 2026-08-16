//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1063/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1063<F: Float>(t37625: F, t37628: F, t565: F, t6481: F, t2111: F, t409: F, t5148: F, t2157: F, t625: F, t1583: F, t2195: F, t573: F) -> (F, F, F, F, F, F) {
    let t37634 = t565 * t37625 * t37628 * t6481;
    let t37637 = t2111 * t409 * t5148;
    let t37638 = t2157 * t625;
    let t37639 = t37637 * t37638;
    let t37641 = t565 * t1583;
    let t37652 = t2195 * t573;
    (t37634, t37637, t37638, t37639, t37641, t37652)
}
