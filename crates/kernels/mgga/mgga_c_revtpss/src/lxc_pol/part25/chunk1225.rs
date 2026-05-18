//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1225/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1225<F: Float>(t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F, t25300: F, t9285: F, t25299: F, t7059: F, t9288: F, t7064: F) -> (F, F, F, F, F, F, F) {
    let t92858 = t2435 * t25352;
    let t92861 = F::new(0.30356481678079769392e-1) * t7018 * t11015;
    let t92864 = t822 * t7048;
    let t92868 = t25300 * t9285;
    let t92870 = F::new(0.68540937416128198417e-2) * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = F::new(0.39982213492741449076e-1) * t7064 * t92871;
    (t92858, t92861, t92864, t92868, t92870, t92871, t92873)
}
