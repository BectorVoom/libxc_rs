//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 910/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk910<F: Float>(t1034: F, t1753: F, t164: F, t179: F, t1721: F, t2639: F, t600: F, t2593: F, t2602: F, t5257: F, t1020: F, t1719: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6859 = t1034 * t1753;
    let t6860 = t6859 * t164;
    let t6861 = t179 * t6860;
    let t6864 = t2639 * t1721;
    let t6865 = t6864 * t600;
    let t6866 = t179 * t6865;
    let t6869 = t2593 * t1753;
    let t6870 = t179 * t6869;
    let t6873 = t5257 * t2602;
    let t6875 = t1020 * t1719;
    (t6859, t6860, t6861, t6864, t6865, t6866, t6869, t6870, t6873, t6875)
}
