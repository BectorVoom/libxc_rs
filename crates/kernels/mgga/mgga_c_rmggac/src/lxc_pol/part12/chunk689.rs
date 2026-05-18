//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 689/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk689<F: Float>(t511: F, t9104: F, t7231: F, t3351: F, t352: F, t618: F, t515: F, t2283: F, t7720: F, t236: F, t495: F, t551: F) -> (F, F, F, F, F, F, F) {
    let t9105 = t511 * t9104;
    let t9106 = t7231 * t9105;
    let t9107 = t3351 * t9106;
    let t9109 = t618 * t352;
    let t9110 = t515 * t9109;
    let t9111 = t7231 * t9110;
    let t9112 = t3351 * t9111;
    let t9114 = t7720 * t2283;
    let t9117 = t236 * t551 * t495;
    (t9106, t9107, t9109, t9111, t9112, t9114, t9117)
}
