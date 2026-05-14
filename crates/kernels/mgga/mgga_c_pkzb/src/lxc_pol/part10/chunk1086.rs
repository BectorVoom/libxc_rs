//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1086/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1086<F: Float>(t2003: F, t3515: F, t655: F, t758: F, t3542: F, t5633: F, t2739: F, t2946: F, t2099: F, t3656: F, t2038: F, t3640: F, t2026: F, t3652: F, t757: F, t179: F, t2068: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9589 = t2003 * t3515;
    let t9590 = t9589 * t655;
    let t9591 = t758 * t9590;
    let t9594 = t5633 * t3542;
    let t9595 = t9594 * t655;
    let t9596 = t758 * t9595;
    let t9599 = t2946 * t2739;
    let t9600 = t758 * t9599;
    let t9605 = t2099 * t3656;
    let t9606 = t2038 * t9605;
    let t9613 = t2099 * t3640;
    let t9614 = t2026 * t9613;
    let t9616 = t2099 * t3652;
    let t9617 = t757 * t9616;
    let t9622 = t179 * t2068 * t3515;
    (t9589, t9590, t9591, t9594, t9595, t9596, t9599, t9600, t9605, t9606, t9613, t9614, t9616, t9617, t9622)
}
