//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 515/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk515<F: Float>(t1034: F, t600: F, t164: F, t179: F, t1020: F, t1041: F, t1769: F, t177: F, t1774: F) -> (F, F, F, F, F, F, F) {
    let t2646 = t1034 * t600;
    let t2647 = t2646 * t164;
    let t2648 = t179 * t2647;
    let t2653 = t1020 * t600;
    let t2654 = t2653 * t164;
    let t2655 = t179 * t2654;
    let t2658 = t1769 * t1041;
    let t2660 = t1774 * t177;
    (t2646, t2647, t2648, t2653, t2655, t2658, t2660)
}
