//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta638<F: Float>(t1388: F, t6324: F, t24994: F, t7684: F, t1307: F, t28830: F, t19534: F, t89: F, t16944: F, t25014: F, t25365: F, t86721: F) -> (F, F, F, F, F, F, F, F) {
        let (t97875, t97890, t97894, t97902, t97911, t97933, t97950, t97953) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1906::<F>(t1388, t6324, t24994, t7684, t1307, t28830, t19534, t89, t16944, t25014, t25365, t86721);
    (t97875, t97890, t97894, t97902, t97911, t97933, t97950, t97953)
}
