//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 861/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk861<F: Float>(t136840: F, t32764: F, t32318: F, t94760: F, t23721: F, t2058: F, t5555: F, t542: F, t133: F, t550: F, t5551: F, t3392: F, t39: F, t39801: F, t40: F, t136825: F, t32774: F, t32775: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t138843 = t32764 * t136840;
    let t138854 = t94760 * t32318;
    let t138857 = t23721 * t32318;
    let t138866 = t2058 * t5555;
    let t138867 = t542 * t138866;
    let t138870 = t133 * t138866;
    let t138873 = t550 * t5551;
    let t138874 = t133 * t138873;
    let t138879 = t3392 * t39801 * t39 * t40;
    let t138888 = t32774 * t136825 * t32775;
    (t138843, t138854, t138857, t138866, t138867, t138870, t138873, t138874, t138879, t138888)
}
