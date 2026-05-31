//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 570/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk570<F: Float>(t2847: F, t4571: F, t6094: F, t6098: F, t6102: F, t291: F, t1610: F, t4590: F, t1609: F, t935: F, t2874: F, t1600: F) -> (F, F, F, F, F, F, F) {
    let t6104 = t2847 + F::cast_from(0.11872222222222222222e-1_f64) * t4571 - F::cast_from(0.11872222222222222222e-1_f64) * t6094 + F::cast_from(0.35616666666666666666e-1_f64) * t6098 - F::cast_from(0.17808333333333333333e-1_f64) * t6102;
    let t6106 = F::cast_from(0.621814e-1_f64) * t6104 * t291;
    let t6108 = F::cast_from(2.0_f64) * t4590 * t1610;
    let t6109 = t1609 * t1609;
    let t6110 = t6109 * t935;
    let t6112 = F::cast_from(2.0_f64) * t2874 * t6110;
    let t6113 = t1600 * t1600;
    (t6104, t6106, t6108, t6109, t6110, t6112, t6113)
}
