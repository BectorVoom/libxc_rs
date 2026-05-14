//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 888/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk888<F: Float>(t1209: F, t3727: F, t460: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t459: F, t1294: F, t3790: F) -> (F, F, F, F) {
    let t12666 = t1209 * t3727;
    let t12673 = t460 * t3727;
    let t12678 = 0.25925925925925925926e-1 * t12295;
    let t12689 = -t12678 + 0.11111111111111111111e-1 * t12297 + 0.55555555555555555555e-2 * t12299 - 0.16666666666666666667e-1 * t12301 - 0.83333333333333333334e-2 * t12303 + 0.92592592592592592592e-2 * t12307 - 0.33333333333333333333e-1 * t12310 - 0.16666666666666666666e-1 * t12292 + 0.50000000000000000001e-1 * t12314 + 0.50000000000000000001e-1 * t12317 + 0.83333333333333333333e-2 * t12320;
    let t12690 = t12689 * t459;
    let t12695 = t1294 * t3790;
    (t12666, t12673, t12690, t12695)
}
