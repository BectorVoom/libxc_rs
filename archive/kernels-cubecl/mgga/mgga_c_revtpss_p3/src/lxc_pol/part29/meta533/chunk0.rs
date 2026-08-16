//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1864/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1864<F: Float>(t26435: F, t9303: F, t26440: F, t686: F, t72: F, t25375: F, t2470: F, t26543: F, t7058: F, t122: F, t25412: F, t7398: F) -> (F, F, F, F, F, F) {
    let t95569 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t26435;
    let t95571 = t26440 * t72 * t686;
    let t95572 = t25375 * t95571;
    let t95575 = t26543 * t2470;
    let t95576 = t7058 * t95575;
    let t95593 = t7398 * t72 * t122 * t25412;
    (t95569, t95571, t95572, t95575, t95576, t95593)
}
