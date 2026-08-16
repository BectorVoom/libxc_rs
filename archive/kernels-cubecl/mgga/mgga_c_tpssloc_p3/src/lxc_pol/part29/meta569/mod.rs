//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta569<F: Float>(t13030: F, t225: F, t13062: F, t13378: F, t193: F, t2379: F, t15823: F, t15800: F, t15808: F, t15814: F, t15831: F, t15816: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47585, t47609, t47618, t47645, t51925, t51928, t51937, t52386, t53658, t53703) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1986::<F>(t13030, t225, t13062, t13378, t193, t2379, t15823, t15800, t15808, t15814, t15831, t15816);
    (t47585, t47609, t47618, t47645, t51925, t51928, t51937, t52386, t53658, t53703)
}
