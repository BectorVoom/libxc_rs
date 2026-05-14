//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 798/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk798<F: Float>(t2999: F, t89: F, t943: F, t3000: F, t921: F, t8417: F, t971: F, t8326: F, t8216: F, t1786: F, t3238: F, t3281: F, t981: F, t7943: F, t973: F, t955: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46256 = t89 * t2999 * t943;
    let t46320 = t89 * t3000 * t921;
    let t46565 = t971 * t8417;
    let t47222 = t8326 * t971;
    let t47273 = t8216 * t971;
    let t47443 = t1786 * t3238;
    let t47727 = t3281 * t981;
    let t47836 = t89 * t7943 * t973;
    let t47860 = t3281 * t955;
    (t46256, t46320, t46565, t47222, t47273, t47443, t47727, t47836, t47860)
}
