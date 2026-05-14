//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1251/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1251<F: Float>(t24570: F, t24594: F, t98: F, t126: F, t83: F, t545: F, t8748: F, t16586: F, t1532: F, t3380: F, t49: F, t1639: F, t8770: F, t16593: F, t16595: F, t16600: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24596 = (t24570 + t24594) * t98;
    let t24598 = t83 * t24596 * t126;
    let t24600 = t83 * t8748 * t545;
    let t24601 = 2.0 * t24600;
    let t24602 = 20.0 * t16586;
    let t24604 = t3380 * t49 * t1532;
    let t24605 = 0.10843581300301739842e-1 * t24604;
    let t24606 = t8770 * t1639;
    let t24607 = 0.11696447245269292414e1 * t24606;
    let t24608 = 0.70178683471615754484e1 * t16593;
    let t24609 = 0.43374325201206959368e-1 * t16595;
    let t24610 = 0.65061487801810439052e-1 * t16600;
    (t24596, t24598, t24601, t24602, t24605, t24607, t24608, t24609, t24610)
}
