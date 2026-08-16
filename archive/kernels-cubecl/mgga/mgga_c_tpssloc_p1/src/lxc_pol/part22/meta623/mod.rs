//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta623<F: Float>(t53452: F, t11727: F, t52835: F, t11832: F, t1706: F, t15734: F, t3490: F, t11789: F, t1227: F, t248: F, t4733: F, t11712: F, t11913: F, t491: F) -> (F, F, F, F, F, F) {
        let (t53453, t53472, t53490, t53516, t53520, t53545) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2157::<F>(t53452, t11727, t52835, t11832, t1706, t15734, t3490, t11789, t1227, t248, t4733, t11712, t11913, t491);
    (t53453, t53472, t53490, t53516, t53520, t53545)
}
