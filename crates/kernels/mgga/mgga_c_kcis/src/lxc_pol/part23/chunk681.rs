//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 681/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk681<F: Float>(t62: F, t8538: F, t8537: F, t752: F, t143: F, t740: F, t647: F, t97: F, t728: F, t2440: F, t2438: F, t2459: F, t2568: F, t126: F, t691: F, t2314: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8539 = t62 * t8538;
    let t8540 = t8537 * t8539;
    let t8541 = t752 * t8540;
    let t8543 = t143 * t740;
    let t8546 = t647 * t97;
    let t8547 = t8546 * t728;
    let t8556 = t2440 * t728;
    let t8557 = t2438 * t8556;
    let t8561 = t728 * t2459;
    let t8562 = t2568 * t8561;
    let t8565 = t126 * t691;
    let t8566 = t8565 * t2314;
    (t8541, t8543, t8546, t8547, t8556, t8557, t8561, t8562, t8565, t8566)
}
