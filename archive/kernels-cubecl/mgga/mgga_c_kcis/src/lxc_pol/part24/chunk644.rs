//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 644/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk644<F: Float>(t7589: F, t7592: F, t7583: F, t137: F, t710: F, t86: F, t748: F, t754: F, t774: F) -> (F, F, F, F, F) {
    let t7593 = t7589 * t7592;
    let t7595 = t7589 * t7583;
    let t7598 = t86 * t710 * t137;
    let t7601 = t86 * t748 * t137;
    let t7603 = t754 * t774;
    (t7593, t7595, t7598, t7601, t7603)
}
