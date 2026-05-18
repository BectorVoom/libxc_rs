//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 607/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk607<F: Float>(t2847: F, t2848: F, t2855: F, t2860: F, t2864: F, t291: F, t910: F, t914: F, t936: F, t287: F, t913: F, t275: F) -> (F, F, F, F, F, F) {
    let t2866 = t2847 + F::new(0.11872222222222222222e-1) * t2848 - F::new(0.11872222222222222222e-1) * t2855 + F::new(0.35616666666666666666e-1) * t2860 - F::new(0.17808333333333333333e-1) * t2864;
    let t2868 = F::new(0.621814e-1) * t2866 * t291;
    let t2869 = t910 * t914;
    let t2871 = F::new(2.0) * t2869 * t936;
    let t2872 = t913 * t287;
    let t2873 = F::new(1.0) / t2872;
    let t2874 = t275 * t2873;
    (t2866, t2868, t2869, t2871, t2873, t2874)
}
