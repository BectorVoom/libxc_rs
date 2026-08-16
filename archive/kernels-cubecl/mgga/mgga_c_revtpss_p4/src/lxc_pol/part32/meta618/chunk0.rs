//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1958/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1958<F: Float>(t22267: F, t25997: F, t22255: F, t7264: F, t22259: F, t22276: F, t7271: F, t22281: F, t26024: F, t6876: F, t22289: F, t22115: F, t26028: F) -> (F, F, F, F, F, F, F, F) {
    let t108566 = t25997 * t22267;
    let t108568 = t7264 * t22255;
    let t108570 = t25997 * t22259;
    let t108572 = t7271 * t22276;
    let t108574 = t7271 * t22281;
    let t108576 = t26024 * t6876;
    let t108578 = t7271 * t22289;
    let t108583 = t26028 * t22115;
    (t108566, t108568, t108570, t108572, t108574, t108576, t108578, t108583)
}
