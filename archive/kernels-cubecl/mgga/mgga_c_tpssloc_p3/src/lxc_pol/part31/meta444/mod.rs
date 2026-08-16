//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1591;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta444<F: Float>(t225: F, t7179: F, t22692: F, t22717: F, t22725: F, t1338: F, t7191: F, t2085: F, t3787: F, t22923: F, t22925: F, t532: F, t7216: F, t193: F, t201: F, t2056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24095, t24099, t24108, t24110, t24116, t24127, t24156, t24157, t24175) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1591::<F>(t225, t7179, t22692, t22717, t22725, t1338, t7191, t2085, t3787, t22923, t22925, t532, t7216);
        let t24191 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1592::<F>(t193, t201, t2056);
    (t24095, t24099, t24108, t24110, t24116, t24127, t24156, t24157, t24175, t24191)
}
