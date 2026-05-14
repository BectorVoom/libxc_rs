//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1102/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1102<F: Float>(t92838: F, t92840: F, t25372: F, t92837: F, t25287: F, t786: F, t789: F, t2829: F, t689: F, t7014: F, t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t92841 = t92838 * t92840;
    let t92843 = t25372 * t92837;
    let t92844 = t92843 * t92840;
    let t92847 = t786 * t25287 * t789;
    let t92856 = t689 * t7014 * t2829;
    let t92858 = t2435 * t25352;
    let t92861 = 0.30356481678079769392e-1 * t7018 * t11015;
    let t92864 = t822 * t7048;
    (t92841, t92844, t92847, t92856, t92858, t92861, t92864)
}
