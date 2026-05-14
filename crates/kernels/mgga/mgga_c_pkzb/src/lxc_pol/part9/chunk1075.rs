//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1075/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1075<F: Float>(t1029: F, t160: F, t1634: F, t1692: F, t1747: F, t1750: F, t1773: F, t19867: F, t20397: F, t2575: F, t2625: F, t2631: F, t5357: F, t5361: F, t568: F, t596: F, t614: F, t6853: F, t7065: F, t7074: F, t7075: F, t7078: F, t8865: F) -> (F,) {
    let t20398 = 180.0 * t1634 * t1773 * t2575 * t2631 - 36.0 * t2631 * t568 * t614 * t6853 + 3.0 * t160 * t19867 * t596 - 36.0 * t1692 * t2631 * t7074 + 60.0 * t1029 * t5357 - 36.0 * t1747 * t2625 + 9.0 * t1750 * t2625 - 36.0 * t5361 * t8865 - 72.0 * t7065 * t7075 - 36.0 * t7065 * t7078 + t20397;
    (t20398,)
}
