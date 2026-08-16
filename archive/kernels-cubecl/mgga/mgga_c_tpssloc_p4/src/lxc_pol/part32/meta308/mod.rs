//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta308<F: Float>(t1020: F, t10510: F, t2928: F, t320: F, t10294: F, t268: F, t271: F, t6546: F, t2394: F, t885: F) -> (F, F, F, F, F, F) {
        let (t10511, t10523, t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1334::<F>(t1020, t10510, t2928, t320, t10294, t268, t271, t6546, t2394, t885);
    (t10511, t10523, t10542, t10544, t10545, t10556)
}
