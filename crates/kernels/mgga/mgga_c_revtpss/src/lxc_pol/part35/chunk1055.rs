//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1055/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1055<F: Float>(t22895: F, t26028: F, t22837: F, t22843: F, t27940: F, t22833: F, t22914: F, t7264: F, t22865: F, t25983: F, t22860: F, t94493: F, t22854: F, t7271: F, t22956: F, t22822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t114547 = t26028 * t22895;
    let t114549 = t26028 * t22837;
    let t114551 = t27940 * t22843;
    let t114553 = t27940 * t22833;
    let t114564 = t7264 * t22914;
    let t114566 = t25983 * t22865;
    let t114573 = t94493 * t22860;
    let t114575 = t7271 * t22854;
    let t114577 = t7264 * t22956;
    let t114584 = t7271 * t22822;
    (t114547, t114549, t114551, t114553, t114564, t114566, t114573, t114575, t114577, t114584)
}
