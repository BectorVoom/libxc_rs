//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 716/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk716<F: Float>(t62: F, t8750: F, t755: F, t752: F, t2479: F, t754: F, t775: F, t2724: F, t870: F, t2726: F, t887: F, t217: F, t2727: F) -> (F, F, F, F, F, F) {
    let t8751 = t62 * t8750;
    let t8752 = t755 * t8751;
    let t8753 = t752 * t8752;
    let t8755 = t2479 * t754;
    let t8756 = t8755 * t775;
    let t8757 = t752 * t8756;
    let t8759 = t870 * t2724;
    let t8762 = t2726 * t887;
    let t8764 = F::cast_from(1.0_f64) / t2727 / t217;
    (t8753, t8755, t8757, t8759, t8762, t8764)
}
