//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1280/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1280<F: Float>(t11190: F, t2203: F, t836: F, t11184: F, t18492: F, t3046: F, t9805: F, t3747: F, t7972: F, t3052: F, t9798: F, t2215: F) -> (F, F, F, F, F, F) {
    let t31213 = t2203 * t11190 * t836;
    let t31216 = t18492 * t11184 * t836;
    let t31218 = t9805 * t3046;
    let t31220 = t7972 * t3747;
    let t31222 = t3052 * t9798;
    let t31225 = t2215 * t11190 * t836;
    (t31213, t31216, t31218, t31220, t31222, t31225)
}
