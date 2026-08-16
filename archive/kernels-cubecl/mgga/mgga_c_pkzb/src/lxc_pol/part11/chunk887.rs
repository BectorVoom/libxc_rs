//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 887/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk887<F: Float>(t2003: F, t3515: F, t655: F, t758: F, t3542: F, t5633: F, t2739: F, t2946: F, t2099: F, t3656: F, t2038: F, t3640: F) -> (F, F, F, F, F, F, F, F, F, F) {
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
    (t9590, t9591, t9594, t9595, t9596, t9599, t9600, t9605, t9606, t9613)
}
