//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2061/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061<F: Float>(t90514: F, t1377: F, t5187: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F) -> (F, F, F, F, F, F) {
    let t90515 = F::cast_from(0.82246703342411321824e-2_f64) * t90514;
    let t90516 = t1377 * t5187;
    let t90521 = t81186 * t7692;
    let t90524 = t81228 * t81326 * t26338;
    let t90525 = F::cast_from(0.16449340668482264365e-1_f64) * t90524;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = F::cast_from(0.16449340668482264365e-1_f64) * t90533;
    let t90541 = t6883 * t26206;
    (t90515, t90516, t90521, t90525, t90534, t90541)
}
