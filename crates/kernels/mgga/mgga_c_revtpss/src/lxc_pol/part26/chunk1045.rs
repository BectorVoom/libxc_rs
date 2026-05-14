//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1045/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1045<F: Float>(t25372: F, t95536: F, t92840: F, t7398: F, t822: F, t25375: F, t95765: F, t25411: F, t95597: F, t93170: F, t95746: F, t26446: F, t689: F, t887: F, t26481: F, t2724: F, t676: F) -> (F, F, F, F, F, F, F) {
    let t95822 = t25372 * t95536;
    let t95823 = t95822 * t92840;
    let t95825 = t822 * t7398;
    let t95832 = t25375 * t95765;
    let t95834 = t25411 * t95597;
    let t95836 = t93170 * t95746;
    let t95847 = t689 * t26446 * t887;
    let t95854 = t26481 * t676 * t2724;
    (t95823, t95825, t95832, t95834, t95836, t95847, t95854)
}
