//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 827/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk827<F: Float>(t336: F, t9617: F, t578: F, t1734: F, t579: F, t2046: F, t1795: F, t599: F, t137: F, t1894: F, t2263: F, t8480: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9618 = t336 * t9617;
    let t9619 = t578 * t9618;
    let t9621 = t579 * t1734;
    let t9622 = t336 * t9621;
    let t9623 = t2046 * t9622;
    let t9625 = t599 * t1795;
    let t9626 = t336 * t9625;
    let t9627 = t578 * t9626;
    let t9630 = t336 * t1894 * t137;
    let t9631 = t578 * t9630;
    let t9633 = t8480 * t2263;
    (t9618, t9619, t9622, t9623, t9626, t9627, t9630, t9631, t9633)
}
