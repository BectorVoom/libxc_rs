//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1181/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1181<F: Float>(t4868: F, t7046: F, t2543: F, t500: F, t2551: F, t1545: F, t2605: F, t2609: F, t5089: F, t135: F, t568: F, t5146: F, t1542: F, t1020: F, t1816: F, t1009: F, t4803: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19627 = t7046 * t4868;
    let t19653 = 32.0 * t2543 * t500;
    let t19680 = 32.0 * t2551 * t500;
    let t19687 = t1545 * t2605;
    let t19702 = t2609 * t5089;
    let t19704 = t135 * t568;
    let t19710 = t2609 * t5146;
    let t19742 = t1542 * t2605;
    let t19744 = t1020 * t1816;
    let t19754 = t4803 * t1009;
    (t19627, t19653, t19680, t19687, t19702, t19704, t19710, t19742, t19744, t19754)
}
