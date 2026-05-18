//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 828/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk828<F: Float>(t1713: F, t579: F, t336: F, t7400: F, t1782: F, t604: F, t578: F, t1734: F, t2046: F, t1795: F, t599: F, t137: F, t1894: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9613 = t579 * t1713;
    let t9614 = t336 * t9613;
    let t9615 = t7400 * t9614;
    let t9617 = t604 * t1782;
    let t9618 = t336 * t9617;
    let t9619 = t578 * t9618;
    let t9621 = t579 * t1734;
    let t9622 = t336 * t9621;
    let t9623 = t2046 * t9622;
    let t9625 = t599 * t1795;
    let t9626 = t336 * t9625;
    let t9627 = t578 * t9626;
    let t9630 = t336 * t1894 * t137;
    (t9614, t9615, t9618, t9619, t9622, t9623, t9626, t9627, t9630)
}
