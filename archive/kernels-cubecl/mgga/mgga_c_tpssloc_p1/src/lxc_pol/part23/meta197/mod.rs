//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta197<F: Float>(t11545: F, t974: F, t11147: F, t461: F, t457: F, t63: F, t221: F, t456: F, t3242: F, t460: F, t3247: F, t1176: F, t134: F) -> (F, F, F, F, F, F, F, F) {
        let (t11546, t11547, t11552, t11554, t11556, t11570, t11583, t11588) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk835::<F>(t11545, t974, t11147, t461, t457, t63, t221, t456, t3242, t460, t3247, t1176, t134);
    (t11546, t11547, t11552, t11554, t11556, t11570, t11583, t11588)
}
