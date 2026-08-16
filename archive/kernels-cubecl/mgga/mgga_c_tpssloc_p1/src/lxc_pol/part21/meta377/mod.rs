//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta377<F: Float>(t10255: F, t4531: F, t343: F, t4540: F, t984: F, t4546: F, t12606: F, t978: F, t977: F, t135: F, t340: F) -> (F, F, F, F, F, F) {
        let (t13806, t13812, t13813, t13816, t13817, t13822) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1829::<F>(t10255, t4531, t343, t4540, t984, t4546, t12606, t978, t977, t135, t340);
    (t13806, t13812, t13813, t13816, t13817, t13822)
}
