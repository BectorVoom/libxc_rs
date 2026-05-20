//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2229/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229<F: Float>(t21804: F, t76: F, t2242: F, t5819: F, t38: F, t60670: F, t1923: F, t1926: F, t1928: F, t28078: F, t28089: F, t28093: F, t29513: F, t29532: F, t29533: F, t29551: F, t6954: F, t6973: F, t6974: F, t6978: F, t7702: F, t7715: F, t7716: F) -> F {
    let t108941 = t76 * t21804;
    let t108945 = t2242 * t5819;
    let t108952 = t60670 * t38;
    let t108963 = -t1923 * t7715 * t28089 / F::new(3.0) - t6954 * t29533 / F::new(6.0) - t1923 * t6973 * t29532 / F::new(6.0) - t1923 * t1926 * t108941 / F::new(6.0) + t108945 * t1928 / F::new(3.0) + t29551 * t6974 / F::new(3.0) + t29551 * t6978 / F::new(3.0) - t108952 * t1928 / F::new(6.0) - t29513 * t6974 / F::new(6.0) - t29513 * t6978 / F::new(6.0) - t28093 * t7716 / F::new(3.0) - t7702 * t28078 / F::new(3.0);
    t108963
}
