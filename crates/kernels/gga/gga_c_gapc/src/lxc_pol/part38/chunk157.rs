//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 157/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk157<F: Float>(t520: F, t522: F, t429: F, t472: F, t152: F, t203: F, t101: F, t9: F, t22: F, t423: F, t431: F, t457: F, t159: F, t405: F, t104: F, t73: F) -> (F, F, F, F, F, F, F, F) {
    let t523 = t520 * t522;
    let t526 = t429 * t472;
    let t527 = t152 * t203;
    let t528 = t9 * t101;
    let t532 = t22 * t423;
    let t536 = t431 * t457;
    let t540 = t405 * t159;
    let t543 = t73 * t104;
    (t523, t526, t527, t528, t532, t536, t540, t543)
}
