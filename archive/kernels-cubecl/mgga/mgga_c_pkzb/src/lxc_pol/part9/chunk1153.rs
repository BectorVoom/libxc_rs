//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1153/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1153<F: Float>(t5373: F, t6897: F, t1020: F, t164: F, t600: F, t7084: F, t5257: F, t6958: F, t1034: F, t5367: F, t1753: F, t2639: F) -> (F, F, F, F, F, F) {
    let t20071 = t6897 * t5373;
    let t20075 = t1020 * t5373;
    let t20081 = t7084 * t600 * t164;
    let t20085 = t5257 * t6958;
    let t20093 = t1034 * t5367 * t164;
    let t20102 = t2639 * t1753 * t164;
    (t20071, t20075, t20081, t20085, t20093, t20102)
}
