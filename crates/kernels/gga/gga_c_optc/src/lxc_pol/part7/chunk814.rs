//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 814/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk814<F: Float>(t2760: F, t888: F, t2758: F, t2753: F, t2751: F, t140: F, t2665: F, t883: F, t2661: F, t2748: F, t7878: F, t942: F, t940: F, t2708: F, t2754: F, t2761: F, t2766: F, t324: F, t8226: F, t8231: F, t8233: F, t8236: F, t8241: F) -> (F, F, F, F, F, F) {
    let t8243 = t888 * t2760;
    let t8244 = t2758 * t8243;
    let t8246 = t888 * t2753;
    let t8247 = t2751 * t8246;
    let t8250 = t883 * t2665 * t140;
    let t8251 = t2661 * t8250;
    let t8254 = t2748 * t8250;
    let t8257 = t7878 * t942;
    let t8258 = t940 * t8257;
    let t8260 = 0.15454509315180013964e0 * t8226 + t8231 - 0.39666573908962035841e1 * t8233 * t324 + 0.84999801233490076802e0 * t8236 - 0.9356877183176434872e2 * t2708 * t2766 + 0.1169609647897054359e2 * t8241 - 0.15486228121497046737e2 * t8244 + 0.4645868436449114021e2 * t8247 + 0.12388982497197637389e3 * t8251 * t2761 - 0.37166947491592912168e3 * t8254 * t2754 - 0.389869882632351453e1 * t8258;
    (t8243, t8246, t8251, t8254, t8257, t8260)
}
