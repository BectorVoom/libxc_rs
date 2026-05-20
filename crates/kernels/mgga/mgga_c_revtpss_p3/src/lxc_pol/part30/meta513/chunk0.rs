//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1901/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1901<F: Float>(t7769: F, t886: F, t25317: F, t225: F, t27265: F, t1579: F, t231: F, t836: F, t25392: F, t7048: F, t7071: F, t7759: F) -> (F, F, F, F, F, F, F, F) {
    let t27299 = t7769 * t886;
    let t27300 = t25317 * t27299;
    let t27303 = t27265 * t225;
    let t27312 = t1579 * t836 * t231;
    let t27313 = t25392 * t27312;
    let t27316 = t7048 * t1579;
    let t27317 = t7071 * t27316;
    let t27322 = t7071 * t7759 * t886;
    (t27299, t27300, t27303, t27312, t27313, t27316, t27317, t27322)
}
