//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1291/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1291<F: Float>(t33808: F, t9532: F, t32401: F, t9860: F, t33854: F, t33830: F, t3936: F, t115078: F, t33937: F, t33928: F, t33802: F, t32388: F, t9851: F, t33873: F, t9529: F, t33851: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115384 = 0.34722222222222222222e-2 * t33808 * t9532;
    let t115393 = 0.34722222222222222222e-2 * t9860 * t32401;
    let t115404 = 0.34722222222222222222e-2 * t33854 * t9532;
    let t115416 = t3936 * t33830;
    let t115426 = t33937 * t115078;
    let t115430 = 0.34722222222222222222e-2 * t33928 * t9532;
    let t115433 = t33802 * t9532;
    let t115454 = t9851 * t32388;
    let t115463 = t9529 * t33873;
    let t115468 = t33851 * t9532;
    (t115384, t115393, t115404, t115416, t115426, t115430, t115433, t115454, t115463, t115468)
}
