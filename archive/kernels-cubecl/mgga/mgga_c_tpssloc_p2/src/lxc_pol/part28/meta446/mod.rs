//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1631;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1632;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta446<F: Float>(t13487: F, t23788: F, t1081: F, t776: F, t2553: F, t28: F, t2749: F, t868: F, t2745: F, t12461: F, t3698: F, t2039: F, t3652: F, t109: F, t22468: F, t22471: F, t22474: F, t22476: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23789, t23792, t23796, t23807, t23810, t23813, t23857, t23909) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1631::<F>(t13487, t23788, t1081, t776, t2553, t28, t2749, t868, t2745, t12461, t3698, t2039, t3652);
        let (t23912, t23917) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1632::<F>(t109, t22468, t22471, t22474, t22476);
    (t23789, t23792, t23796, t23807, t23810, t23813, t23857, t23909, t23912, t23917)
}
