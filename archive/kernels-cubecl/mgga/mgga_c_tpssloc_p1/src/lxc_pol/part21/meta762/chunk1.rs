//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2637/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2637<F: Float>(t1336: F, t1361: F, t242: F, t12189: F, t5206: F, t40406: F, t5202: F, t16115: F, t3726: F, t12199: F, t16111: F, t1804: F, t40005: F) -> (F, F, F, F, F, F) {
    let t54614 = t1336 * t1361 * t242;
    let t54631 = t12189 * t5206;
    let t54633 = t40406 * t5202;
    let t54635 = t3726 * t16115;
    let t54637 = t12199 * t16111;
    let t54639 = t40005 * t1804;
    (t54614, t54631, t54633, t54635, t54637, t54639)
}
