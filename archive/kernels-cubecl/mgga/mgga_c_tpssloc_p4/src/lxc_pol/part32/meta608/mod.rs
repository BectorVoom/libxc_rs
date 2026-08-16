//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2005;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta608<F: Float>(t225: F, t814: F, t6648: F, t81612: F, t22715: F, t6551: F, t6640: F, t117: F, t4179: F, t6559: F, t229: F, t268: F, t2627: F, t6624: F, t131: F, t2587: F, t81142: F, t1905: F, t9537: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81613, t81615, t81632, t81633, t81640, t81651) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2005::<F>(t225, t814, t6648, t81612, t22715, t6551, t6640, t117, t4179, t6559, t229, t268);
        let (t81679, t81686, t81689, t81715, t81717, t81735) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2006::<F>(t2627, t6624, t131, t2587, t81142, t1905, t9537, t81151, t23172, t133, t1891, t6601, t80953);
    (t81613, t81615, t81632, t81633, t81640, t81651, t81679, t81686, t81689, t81715, t81717, t81735)
}
