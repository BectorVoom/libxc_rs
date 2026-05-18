//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 821/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk821<F: Float>(t1020: F, t637: F, t6819: F, t6821: F, t6826: F, t135: F, t1535: F, t192: F, t2536: F, t2537: F, t2714: F, t2718: F, t3401: F, t4867: F, t4870: F, t4876: F, t4879: F, t4886: F, t5077: F, t568: F, t6758: F, t6803: F, t6810: F, t6813: F, t8716: F, t8719: F, t8720: F, t8750: F, t8751: F) -> (F, F, F, F, F) {
    let t8758 = t1020 * t637;
    let t8762 = F::new(40.0) * t6819;
    let t8763 = F::new(24.0) * t6821;
    let t8764 = F::new(2.0) * t6826;
    let t8768 = F::new(6.0) * t135 * t192 * t3401 * t568 - F::new(6.0) * t1535 * t2537 * t8758 - t2536 * t637 * t8751 + F::new(12.0) * t2714 * t2718 * t6758 + t4867 + t4870 - t4876 - t4879 + t4886 + t5077 - t6803 - t6810 - t6813 + t8716 - t8719 + t8720 + t8750 + t8762 - t8763 + t8764;
    (t8758, t8762, t8763, t8764, t8768)
}
