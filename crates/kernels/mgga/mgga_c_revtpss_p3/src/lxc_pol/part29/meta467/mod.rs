//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta467<F: Float>(t2061: F, t785: F, t780: F, t2439: F, t2435: F, t7385: F, t2828: F, t7071: F, t212: F, t7398: F, t689: F, t25219: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26434, t26435, t26437, t26439, t26440, t26441, t26446, t26447, t26448, t26450) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1724::<F>(t2061, t785, t780, t2439, t2435, t7385, t2828, t7071, t212, t7398, t689, t25219);
    (t26434, t26435, t26437, t26439, t26440, t26441, t26446, t26447, t26448, t26450)
}
