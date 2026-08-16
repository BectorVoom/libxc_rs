//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1189;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta302<F: Float>(t2791: F, t888: F, t2929: F, t938: F, t10523: F, t315: F, t10544: F, t1043: F, t676: F, t248: F, t884: F, t1041: F, t10478: F, t3128: F, t10472: F, t1015: F, t1030: F, t3036: F, t3033: F, t698: F, t999: F, t973: F, t363: F, t3068: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10817, t10825, t10828, t10832, t10868, t10871) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1189::<F>(t2791, t888, t2929, t938, t10523, t315, t10544, t1043, t676, t248, t884, t1041);
        let (t10876, t10883, t10891, t10904, t10923, t10936) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1190::<F>(t10478, t3128, t10472, t1015, t1030, t3036, t3033, t698, t999, t973, t363, t3068);
    (t10817, t10825, t10828, t10832, t10868, t10871, t10876, t10883, t10891, t10904, t10923, t10936)
}
