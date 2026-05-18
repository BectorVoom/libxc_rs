//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1208/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1208<F: Float>(t10496: F, t17245: F, t3396: F, t637: F, t3401: F, t5165: F, t1535: F, t1536: F, t16701: F, t16873: F, t2536: F, t2537: F, t2706: F, t2718: F, t29093: F, t29119: F, t29120: F, t29122: F, t29124: F, t568: F, t8751: F) -> (F, F) {
    let t29666 = t10496 * t17245;
    let t29677 = t3396 * t637;
    let t29684 = t3401 * t637;
    let t29691 = t10496 * t5165;
    let t29695 = -F::new(9.0) * t1535 * t2537 * t29677 + F::new(6.0) * t1535 * t29691 * t568 + F::new(18.0) * t1536 * t2718 * t29093 - F::new(3.0) * t2536 * t2706 * t8751 - F::new(18.0) * t2537 * t2718 * t29684 + t16701 + t16873 - t29119 + t29120 + t29122 - t29124;
    (t29666, t29695)
}
