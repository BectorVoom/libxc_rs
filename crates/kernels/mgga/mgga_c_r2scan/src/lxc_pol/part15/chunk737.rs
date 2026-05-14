//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 737/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk737<F: Float>(t6957: F, t1527: F, t2788: F, t4983: F, t2461: F, t879: F, t4721: F, t4964: F, t4967: F, t4972: F, t4975: F, t4979: F, t4981: F, t6954: F, t2321: F, t955: F) -> (F, F, F, F) {
    let t6958 = 6.0 * t6957;
    let t6959 = t2788 * t1527;
    let t6960 = 0.10843581300301739842e-1 * t6959;
    let t6961 = 48.0 * t4983;
    let t6963 = 2.0 * t879 * t2461;
    let t6964 = -t4721 + t4964 - t4967 - t6954 - t4972 + t4975 - t6958 - t6960 + t4979 + t4981 - t6961 + t6963;
    let t6966 = t2321 * t955;
    (t6960, t6961, t6964, t6966)
}
