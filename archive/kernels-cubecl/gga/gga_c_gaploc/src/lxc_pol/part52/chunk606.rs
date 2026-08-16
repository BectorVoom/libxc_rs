//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 606/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk606<F: Float>(t1628: F, t3585: F, t3576: F, t3556: F, t524: F, t3560: F, t11218: F, t600: F, t568: F, t11254: F, t447: F, t1445: F) -> (F, F, F, F, F, F) {
    let t11501 = t1628 * t3585;
    let t11504 = t1628 * t3576;
    let t11513 = t524 * t3556;
    let t11516 = t524 * t3560;
    let t11523 = t600 * t11218;
    let t11524 = t568 * t11523;
    let t11527 = t11254 * t447;
    let t11528 = t1445 * t11527;
    (t11501, t11504, t11513, t11516, t11524, t11528)
}
