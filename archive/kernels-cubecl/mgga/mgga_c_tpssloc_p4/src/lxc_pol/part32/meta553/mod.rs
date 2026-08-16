//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta553<F: Float>(t1268: F, t28017: F, t510: F, t652: F, t7685: F, t7756: F, t5493: F, t89: F, t1874: F, t7458: F, t7461: F, t4028: F, t7468: F) -> (F, F, F, F, F, F, F, F) {
        let (t28019, t28025, t28027, t28029, t28030, t28032, t28034, t28036) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1913::<F>(t1268, t28017, t510, t652, t7685, t7756, t5493, t89, t1874, t7458, t7461, t4028, t7468);
    (t28019, t28025, t28027, t28029, t28030, t28032, t28034, t28036)
}
