//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1074/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1074<F: Float>(t37718: F, t37721: F, t39632: F, t39635: F, t39645: F, t39647: F, t39650: F, t41474: F, t41475: F, t41478: F, t41479: F, t41480: F, t37749: F, t37759: F, t37762: F, t39655: F, t39658: F, t39661: F, t39664: F, t39667: F, t39669: F, t39672: F, t39674: F, t39677: F) -> (F, F) {
    let t41484 = -0.95219938395347901946e-2 * t37718 - 0.28565981518604370584e-1 * t37721 + t41474 + t41475 + 0.52396431978519890152e-1 * t39632 - 0.25426783770825854453e1 * t39635 - t41478 - t41479 + t41480 + 0.52009330440325611378e0 * t39645 + 0.32927245914677557992e0 * t39647 - 0.13099107994629972538e-1 * t39650;
    let t41498 = -0.13099107994629972538e-1 * t39655 + 0.87327386630866483588e-2 * t39658 - 0.2600466522016280569e0 * t39661 - 0.34672886960217074252e0 * t39664 + 0.13099107994629972538e-1 * t39667 - 0.86682217400542685632e-1 * t39669 - 0.13869154784086829701e1 * t37749 - 0.51220160311720645767e0 * t39672 + 0.5200933044032561138e0 * t39674 - 0.5200933044032561138e0 * t39677 - 0.23804984598836975486e0 * t37759 + 0.47609969197673950973e-2 * t37762;
    (t41484, t41498)
}
