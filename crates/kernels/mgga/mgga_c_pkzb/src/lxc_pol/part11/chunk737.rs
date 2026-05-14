//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 737/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk737<F: Float>(t326: F, t6455: F, t401: F, t5722: F, t46: F, t394: F, t5728: F, t5939: F, t922: F, t918: F, t54: F, t931: F) -> (F, F, F, F, F, F, F, F) {
    let t6456 = t6455 * t326;
    let t6457 = t401 * t5722;
    let t6458 = t6457 * t46;
    let t6459 = t6456 * t6458;
    let t6462 = t5728 * t394;
    let t6467 = t5939 * t922;
    let t6468 = t918 * t6467;
    let t6475 = t54 * t931;
    (t6456, t6457, t6458, t6459, t6462, t6467, t6468, t6475)
}
