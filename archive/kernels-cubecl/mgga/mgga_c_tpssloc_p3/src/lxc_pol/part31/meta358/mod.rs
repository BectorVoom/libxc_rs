//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta358<F: Float>(t15908: F, t2375: F, t12045: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F, t1787: F, t2516: F, t12120: F, t2663: F, t5157: F) -> (F, F, F, F, F, F, F, F) {
        let (t15909, t15911, t15916, t15917, t15923, t15972, t15976, t15979) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1275::<F>(t15908, t2375, t12045, t12052, t12054, t5151, t750, t17, t1787, t2516, t12120, t2663, t5157);
    (t15909, t15911, t15916, t15917, t15923, t15972, t15976, t15979)
}
