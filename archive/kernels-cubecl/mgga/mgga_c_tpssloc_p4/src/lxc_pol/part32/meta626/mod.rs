//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2035;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta626<F: Float>(t86928: F, t6562: F, t7488: F, t82133: F, t25225: F, t6547: F, t23168: F, t25338: F, t23012: F, t7485: F, t25046: F, t6579: F, t1484: F, t2717: F, t225: F, t25051: F, t7489: F, t23164: F, t23204: F, t25341: F, t1887: F, t81956: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86929, t86941, t86943, t86951, t86955, t86967) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2035::<F>(t86928, t6562, t7488, t82133, t25225, t6547, t23168, t25338, t23012, t7485, t25046, t6579);
        let (t86968, t86969, t86988, t86991, t87029, t87049) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2036::<F>(t86967, t1484, t2717, t225, t25051, t23012, t7489, t23164, t23204, t25341, t1887, t81956);
    (t86929, t86941, t86943, t86951, t86955, t86968, t86969, t86988, t86991, t87029, t87049)
}
