//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1209/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1209<F: Float>(t39548: F, t39558: F, t37660: F, t39540: F, t39542: F, t39545: F, t39550: F, t39552: F, t39554: F, t39561: F, t39563: F, t39565: F) -> F {
    let t41435 = F::new(0.95219938395347901946e-2) * t39548;
    let t41439 = F::new(0.45022119329691164871e0) * t39558;
    let t41443 = -F::new(0.87327386630866483588e-2) * t39540 - F::new(0.32927245914677557992e0) * t39542 - F::new(0.52009330440325611378e0) * t39545 - F::new(0.28565981518604370584e-1) * t37660 - t41435 - F::new(0.10975748638225852664e0) * t39550 - F::new(0.86682217400542685632e-1) * t39552 - F::new(0.17336443480108537126e0) * t39554 - t41439 - F::new(0.86682217400542685632e-1) * t39561 - F::new(0.2600466522016280569e0) * t39563 - F::new(0.5200933044032561138e0) * t39565;
    t41443
}
