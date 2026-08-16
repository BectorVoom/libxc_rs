//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta510<F: Float>(t23754: F, t2970: F, t23694: F, t3014: F, t23546: F, t2926: F, t3011: F, t24186: F, t3336: F, t11249: F, t23640: F, t15926: F, t19976: F) -> (F, F, F, F, F, F, F) {
        let (t78165, t78207, t78329, t78429, t78478, t78496, t78512) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1525::<F>(t23754, t2970, t23694, t3014, t23546, t2926, t3011, t24186, t3336, t11249, t23640, t15926, t19976);
    (t78165, t78207, t78329, t78429, t78478, t78496, t78512)
}
