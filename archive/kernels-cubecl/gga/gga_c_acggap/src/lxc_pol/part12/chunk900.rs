//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 900/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk900<F: Float>(t1184: F, t30644: F, t7433: F, t7580: F, t7728: F, t1165: F, t3529: F, t7351: F, t7426: F, t7538: F, t7720: F, t7724: F) -> (F, F, F, F, F, F) {
    let t30645 = t30644 * t1184;
    let t30647 = t7433 * t7580;
    let t30649 = t7433 * t7728;
    let t30653 = t7426 * t1165 * t7351 * t3529;
    let t30655 = t7538 * t7720;
    let t30657 = t7538 * t7724;
    (t30645, t30647, t30649, t30653, t30655, t30657)
}
