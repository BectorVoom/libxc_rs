//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 841/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk841<F: Float>(t326: F, t6455: F, t401: F, t5722: F, t46: F, t2368: F, t919: F) -> (F, F, F, F, F) {
    let t6456 = t6455 * t326;
    let t6457 = t401 * t5722;
    let t6458 = t6457 * t46;
    let t6459 = t6456 * t6458;
    let t6460 = t2368 * t919;
    (t6456, t6457, t6458, t6459, t6460)
}
