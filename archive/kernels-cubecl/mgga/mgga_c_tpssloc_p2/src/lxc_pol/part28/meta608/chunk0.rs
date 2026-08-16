//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1917/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1917<F: Float>(t22881: F, t5187: F, t6637: F, t6888: F, t16049: F, t1992: F, t81027: F, t16052: F, t22897: F, t26392: F, t80670: F, t16419: F, t6976: F) -> (F, F, F, F, F) {
    let t90829 = t6888 * t6637 * t22881 * t5187;
    let t90832 = t1992 * t81027 * t16049;
    let t90835 = t1992 * t22897 * t16052;
    let t90837 = t80670 * t26392;
    let t90840 = t1992 * t6976 * t16419;
    (t90829, t90832, t90835, t90837, t90840)
}
