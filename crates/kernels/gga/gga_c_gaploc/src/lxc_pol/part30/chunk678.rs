//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 678/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk678<F: Float>(t1538: F, t6578: F, t6583: F, t1407: F, t2361: F, t203: F, t883: F, t900: F, t4384: F, t2470: F, t549: F, t1416: F, t2367: F, t1429: F, t1265: F, t2366: F) -> (F, F, F, F, F, F) {
    let t6584 = t1538 * t6578;
    let t6585 = t6583 * t6584;
    let t6587 = t1407 * t2361;
    let t6589 = t883 * t203;
    let t6590 = t900 * t6589;
    let t6591 = t4384 * t6590;
    let t6593 = t549 * t2470;
    let t6594 = t1416 * t6593;
    let t6596 = t549 * t2367;
    let t6597 = t1429 * t6596;
    let t6599 = t2366 * t1265;
    (t6585, t6587, t6591, t6594, t6597, t6599)
}
