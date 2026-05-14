//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 492/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk492<F: Float>(t1628: F, t3181: F, t3172: F, t1589: F, t3137: F, t3133: F, t2293: F, t2416: F, t1445: F, t447: F, t9171: F, t590: F, t6519: F, t883: F, t1538: F, t6583: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9511 = t1628 * t3181;
    let t9514 = t1628 * t3172;
    let t9517 = t1589 * t3137;
    let t9520 = t1589 * t3133;
    let t9523 = t2416 * t2293;
    let t9524 = t1445 * t9523;
    let t9527 = t9171 * t447;
    let t9528 = t1445 * t9527;
    let t9531 = t3133 * t590;
    let t9534 = t3137 * t590;
    let t9537 = t883 * t6519;
    let t9538 = t1538 * t9537;
    let t9539 = t6583 * t9538;
    (t9511, t9514, t9517, t9520, t9524, t9528, t9531, t9534, t9537, t9539)
}
