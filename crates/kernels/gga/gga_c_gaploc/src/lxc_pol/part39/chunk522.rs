//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 522/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk522<F: Float>(t701: F, t9603: F, t2580: F, t3270: F, t702: F, t3236: F, t779: F, t1987: F, t3276: F, t3248: F, t731: F, t3240: F, t2549: F, t7221: F, t883: F, t2562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9604 = t9603 * t701;
    let t9605 = t2580 * t9604;
    let t9608 = t3270 * t702;
    let t9611 = t779 * t3236;
    let t9614 = t3276 * t1987;
    let t9618 = 0.85450291446024714264e-3 * t731 * t3248;
    let t9620 = 0.85450291446024714264e-3 * t731 * t3240;
    let t9622 = 0.64087718584518535698e-3 * t2549 * t3248;
    let t9624 = t883 * t7221;
    let t9625 = t2562 * t9624;
    (t9604, t9605, t9608, t9611, t9614, t9618, t9620, t9622, t9624, t9625)
}
