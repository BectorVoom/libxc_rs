//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 827/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk827<F: Float>(t4867: F, t4870: F, t4876: F, t4879: F, t4886: F, t5077: F, t6803: F, t6810: F, t6813: F, t8716: F, t8719: F, t8720: F, t8750: F, t8762: F, t8763: F, t8764: F) -> F {
    let t8839 = t4867 + t4870 + t8716 - t4876 - t4879 - t8719 - t6803 + t8720 + t8750 - t6810 - t6813 + t8762 - t8763 + t8764 + t4886 + t5077;
    t8839
}
