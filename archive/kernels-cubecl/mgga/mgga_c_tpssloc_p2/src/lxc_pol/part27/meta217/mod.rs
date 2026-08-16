//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta217<F: Float>(t1709: F, t3431: F, t1174: F, t3439: F, t60: F, t461: F, t4724: F, t1409: F, t3450: F, t3449: F, t3448: F, t4729: F) -> (F, F, F, F, F, F, F, F) {
        let (t4897, t4899, t4900, t4901, t4904, t4905, t4908, t4909) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1066::<F>(t1709, t3431, t1174, t3439, t60, t461, t4724, t1409, t3450, t3449, t3448, t4729);
    (t4897, t4899, t4900, t4901, t4904, t4905, t4908, t4909)
}
