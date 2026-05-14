//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 683/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk683<F: Float>(t2335: F, t4614: F, t188: F, t6447: F, t1564: F, t2293: F, t475: F, t1445: F, t2304: F, t524: F, t6417: F, t6429: F, t1328: F, t2334: F, t2344: F, t1323: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6637 = t4614 * t2335;
    let t6642 = t188 * t6447;
    let t6647 = t1564 * t2293;
    let t6648 = t6647 * t475;
    let t6649 = t1445 * t6648;
    let t6652 = t524 * t2304;
    let t6655 = t6417 * t475;
    let t6656 = t1445 * t6655;
    let t6659 = t1445 * t6429;
    let t6664 = t2334 * t1328;
    let t6665 = t1445 * t6664;
    let t6668 = t2344 * t1328;
    let t6669 = t1445 * t6668;
    let t6672 = t2344 * t1323;
    (t6637, t6642, t6649, t6652, t6655, t6656, t6659, t6665, t6669, t6672)
}
