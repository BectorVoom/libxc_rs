//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 776/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk776<F: Float>(t218: F, t5631: F, t1527: F, t2718: F, t2728: F, t5585: F, t1510: F, t4295: F, t5612: F, t860: F, t5617: F, t235: F) -> (F, F, F, F, F, F, F, F) {
    let t5632 = t218 * t5631;
    let t5636 = t1527 * t1527;
    let t5637 = t2718 * t5636;
    let t5645 = t2728 * t5585;
    let t5648 = t4295 * t1510;
    let t5651 = t860 * t5612;
    let t5653 = t860 * t5617;
    let t5655 = t235 * t5631;
    (t5632, t5636, t5637, t5645, t5648, t5651, t5653, t5655)
}
