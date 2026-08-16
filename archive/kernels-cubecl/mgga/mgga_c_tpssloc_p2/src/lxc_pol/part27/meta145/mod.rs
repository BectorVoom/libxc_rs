//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk821;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta145<F: Float>(t3040: F, t381: F, t1932: F, t3131: F, t1022: F, t1049: F, t1060: F, t3120: F, t1014: F, t3032: F, t3031: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3187, t3188) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk821::<F>(t3040, t381, t1932, t3131);
        let (t3189, t3192, t3193, t3196, t3197, t3199, t3200) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk822::<F>(t3187, t3188, t1022, t1049, t1060, t3120, t381, t1014, t3032, t3031);
    (t3187, t3188, t3189, t3192, t3193, t3196, t3197, t3199, t3200)
}
