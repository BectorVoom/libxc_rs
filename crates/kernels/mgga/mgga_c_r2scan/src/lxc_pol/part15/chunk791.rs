//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 791/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk791<F: Float>(t4968: F, t2850: F, t797: F, t2266: F, t481: F, t1527: F, t2788: F, t4983: F, t2461: F, t879: F, t4721: F, t4964: F, t4967: F, t4972: F, t4975: F, t4979: F, t4981: F) -> (F, F, F, F) {
    let t6954 = F::new(0.21687162600603479684e-1) * t4968;
    let t6955 = t2850 * t797;
    let t6957 = t2266 * t6955 * t481;
    let t6958 = F::new(6.0) * t6957;
    let t6959 = t2788 * t1527;
    let t6960 = F::new(0.10843581300301739842e-1) * t6959;
    let t6961 = F::new(48.0) * t4983;
    let t6963 = F::new(2.0) * t879 * t2461;
    let t6964 = -t4721 + t4964 - t4967 - t6954 - t4972 + t4975 - t6958 - t6960 + t4979 + t4981 - t6961 + t6963;
    (t6954, t6960, t6961, t6964)
}
