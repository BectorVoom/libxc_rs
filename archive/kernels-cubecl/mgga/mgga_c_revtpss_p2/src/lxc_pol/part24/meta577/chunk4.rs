//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1773/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1773<F: Float>(t81230: F, t81232: F, t81234: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F, t90459: F, t90464: F, t90470: F, t90473: F) -> F {
    let t90717 = -F::cast_from(0.123954e2_f64) * t89828 + F::cast_from(0.3529725e1_f64) * t90459 - F::cast_from(0.27785333333333333333e0_f64) * t81425 + F::cast_from(0.55570666666666666668e0_f64) * t81427 - F::cast_from(0.166712e1_f64) * t81429 + F::cast_from(0.94674375e0_f64) * t90464 - F::cast_from(0.13772666666666666667e1_f64) * t89843 + F::cast_from(0.185931e2_f64) * t89847 + F::cast_from(0.41318e1_f64) * t89855 - F::cast_from(0.13892666666666666667e0_f64) * t90470 - F::cast_from(0.125034e1_f64) * t90473 - F::cast_from(0.76514814814814814814e0_f64) * t81230 + F::cast_from(0.27545333333333333332e1_f64) * t81232 - F::cast_from(0.41318e1_f64) * t81234;
    t90717
}
