//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta181<F: Float>(t3449: F, t4904: F, t3448: F, t461: F, t4729: F, t1178: F, t3966: F, t1177: F, t135: F, t1716: F, t1174: F, t1714: F) -> (F, F, F, F, F, F, F, F) {
        let (t4905, t4908, t4909, t4912, t4913, t4916, t4917, t4919) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk836::<F>(t3449, t4904, t3448, t461, t4729, t1178, t3966, t1177, t135, t1716, t1174, t1714);
    (t4905, t4908, t4909, t4912, t4913, t4916, t4917, t4919)
}
