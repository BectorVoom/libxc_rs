//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta593<F: Float>(t1851: F, t7240: F, t1858: F, t7222: F, t26959: F, t6495: F, t26070: F, t7032: F, t26073: F, t26076: F, t23998: F, t7435: F) -> (F, F, F, F, F, F, F) {
        let (t91834, t91842, t91890, t91894, t91896, t91898, t91900) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1838::<F>(t1851, t7240, t1858, t7222, t26959, t6495, t26070, t7032, t26073, t26076, t23998, t7435);
    (t91834, t91842, t91890, t91894, t91896, t91898, t91900)
}
