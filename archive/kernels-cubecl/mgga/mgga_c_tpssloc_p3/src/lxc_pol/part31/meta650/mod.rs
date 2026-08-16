//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta650<F: Float>(t17037: F, t1888: F, t22996: F, t232: F, t58204: F, t6646: F, t2632: F, t58166: F, t28423: F, t6579: F, t28427: F, t25038: F, t25248: F, t25249: F, t4119: F, t28419: F, t23035: F, t23153: F, t5527: F, t6637: F, t22893: F, t28341: F, t81640: F, t1484: F, t6552: F, t87586: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98478, t98482, t98486, t98488, t98490, t98502) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1926::<F>(t17037, t1888, t22996, t232, t58204, t6646, t2632, t58166, t28423, t6579, t28427, t25038, t25248, t25249, t4119);
        let (t98505, t98513, t98516, t98520) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1927::<F>(t28419, t6579, t23035, t23153, t5527, t6637, t22893, t28341, t81640, t1484, t6552, t87586);
    (t98478, t98482, t98486, t98488, t98490, t98502, t98505, t98513, t98516, t98520)
}
