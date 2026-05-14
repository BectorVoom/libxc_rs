//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 919/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk919<F: Float>(t12018: F, t2268: F, t894: F, t13756: F, t419: F, t13751: F, t380: F, t42625: F, t42629: F, t42633: F, t42637: F, t42638: F, t42641: F, t42645: F, t42648: F, t447: F, t46867: F) -> (F, F) {
    let t46928 = t2268 * t894 * t12018;
    let t46931 = 0.28455006635676149599e-1 * t419 * t13756;
    let t46933 = 0.37940008847568199465e-1 * t380 * t13751;
    let t46935 = 0.28455006635676149599e-1 * t46928 + t46931 - t46933 - 0.56910013271352299198e-1 * t42625 - t42629 - t42633 + t42637 - t42638 + t42641 - t42645 + t42648;
    let t46941 = t46867 * t447;
    (t46935, t46941)
}
