//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1271/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1271<F: Float>(t1058: F, t12639: F, t135: F, t142: F, t1535: F, t1634: F, t1692: F, t1816: F, t19933: F, t24625: F, t24626: F, t24627: F, t24628: F, t24629: F, t24630: F, t24631: F, t2536: F, t2714: F, t2718: F, t6806: F, t8751: F, t9112: F, t9121: F) -> (F,) {
    let t25003 = -24.0 * t1058 * t12639 * t135 * t142 * t6806 - 3.0 * t1535 * t1692 * t9121 + 6.0 * t1634 * t2718 * t9112 - t1816 * t2536 * t8751 + 12.0 * t19933 * t2714 * t2718 - t24625 - t24626 + t24627 + t24628 + t24629 - t24630 + t24631;
    (t25003,)
}
