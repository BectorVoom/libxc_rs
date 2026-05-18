//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1094/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1094<F: Float>(t2609: F, t5146: F, t1542: F, t2605: F, t16613: F, t16619: F, t16621: F, t1009: F, t4803: F, t5142: F, t1639: F, t7035: F) -> (F, F, F, F, F, F, F, F) {
    let t19710 = t2609 * t5146;
    let t19742 = t1542 * t2605;
    let t19743 = F::new(60.0) * t19742;
    let t19748 = F::new(240.0) * t16613;
    let t19751 = F::new(36.0) * t16619;
    let t19752 = F::new(96.0) * t16621;
    let t19754 = t4803 * t1009;
    let t19756 = t5142 * t1009;
    let t19757 = F::new(144.0) * t19756;
    let t19758 = t7035 * t1639;
    (t19710, t19743, t19748, t19751, t19752, t19754, t19757, t19758)
}
