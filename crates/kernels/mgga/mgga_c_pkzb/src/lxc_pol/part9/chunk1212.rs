//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1212/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1212<F: Float>(t4862: F, t4864: F, t5477: F, t6085: F, t6620: F, t6751: F, t6752: F, t6754: F, t7218: F, t7219: F, t7917: F, t23594: F, t4860: F, t5473: F, t5479: F, t5481: F, t6755: F, t6756: F, t7221: F, t7915: F, t8593: F, t9: F) -> (F,) {
    let t23605 = -t6620 + 3.0 * t7218 + t5477 - t6085 - 0.4303125e0 * t7219 + t4864 + 0.1434375e0 * t7917 + t4862 + 3.0 * t6754 + 3.0 * t6751 + 6.0 * t6752;
    let tv4rho41 = -t5479 - 0.7171875e-1 * t8593 + t4860 + 6.0 * t6756 + t5481 - 0.7171875e-1 * t7915 + t9 * t23594 + 3.0 * t6755 + t5473 + 0.286875e0 * t7221 + t23605;
    (tv4rho41,)
}
