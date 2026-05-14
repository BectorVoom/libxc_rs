//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 914/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk914<F: Float>(t4987: F, t8590: F, t88: F, t41: F, t3034: F, t457: F) -> (F, F, F, F, F) {
    let t8634 = 0.17315859105681463759e2 * t4987;
    let t8635 = t8590 * t88;
    let t8636 = t41 * t8635;
    let t8637 = t3034 * t457;
    let t8638 = t41 * t8637;
    (t8634, t8635, t8636, t8637, t8638)
}
