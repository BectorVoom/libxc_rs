//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta311<F: Float>(t10478: F, t3128: F, t10472: F, t1015: F, t1030: F, t3036: F, t3033: F, t698: F, t999: F, t973: F, t363: F, t3068: F) -> (F, F, F, F, F, F, F, F) {
        let (t10876, t10883, t10889, t10891, t10904, t10922, t10923, t10936) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1334::<F>(t10478, t3128, t10472, t1015, t1030, t3036, t3033, t698, t999, t973, t363, t3068);
    (t10876, t10883, t10889, t10891, t10904, t10922, t10923, t10936)
}
