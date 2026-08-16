//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta592<F: Float>(t12915: F, t13487: F, t23788: F, t59580: F, t86815: F, t13196: F, t25891: F, t25927: F, t58009: F, t10143: F, t1081: F, t25374: F) -> (F, F, F, F, F, F) {
        let (t89733, t89837, t89840, t89843, t89846, t89850) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1888::<F>(t12915, t13487, t23788, t59580, t86815, t13196, t25891, t25927, t58009, t10143, t1081, t25374);
    (t89733, t89837, t89840, t89843, t89846, t89850)
}
