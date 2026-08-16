//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta546<F: Float>(t1251: F, t8087: F, t3598: F, t225: F, t497: F, t5052: F, t462: F, t24574: F, t8006: F, t3242: F, t3961: F, t24601: F) -> (F, F, F, F, F, F, F) {
        let (t27761, t27766, t27767, t27770, t27774, t27775, t27776) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1897::<F>(t1251, t8087, t3598, t225, t497, t5052, t462, t24574, t8006, t3242, t3961, t24601);
    (t27761, t27766, t27767, t27770, t27774, t27775, t27776)
}
