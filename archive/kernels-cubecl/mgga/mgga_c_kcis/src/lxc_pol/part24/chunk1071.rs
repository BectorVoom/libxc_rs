//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1071/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1071<F: Float>(t26930: F, t5073: F, t26896: F, t5048: F, t1796: F, t283: F, t7755: F, t1817: F, t2825: F, t4772: F, t5047: F, t7748: F) -> (F, F, F, F, F, F, F) {
    let t28041 = t26930 * t5073;
    let t28043 = t26896 * t5048;
    let t28045 = t1796 * t283;
    let t28046 = t28045 * t7755;
    let t28048 = t2825 * t1817;
    let t28050 = t5047 * t4772;
    let t28051 = t7748 * t28050;
    (t28041, t28043, t28045, t28046, t28048, t28050, t28051)
}
