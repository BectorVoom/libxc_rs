//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 983/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk983<F: Float>(t7618: F, t7686: F, t7749: F, t7802: F, t158: F, t1143: F, t2119: F, t6000: F, t2118: F, t2989: F, t799: F, t2145: F, t2964: F) -> (F, F, F, F, F, F) {
    let t7804 = t7618 + t7686 + t7749 + t7802;
    let t7805 = t7804 * t158;
    let t7821 = t6000 * t1143 * t2119;
    let t7824 = t2118 * t2989;
    let t7825 = t7824 * t799;
    let t7828 = t2964 * t2145;
    (t7804, t7805, t7821, t7824, t7825, t7828)
}
