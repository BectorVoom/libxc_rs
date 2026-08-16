//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1765;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta543<F: Float>(t22690: F, t23153: F, t23171: F, t6561: F, t80741: F, t6643: F, t23025: F, t23030: F, t23012: F, t6653: F, t22641: F, t2588: F, t225: F, t814: F, t6648: F, t22715: F, t6551: F, t6640: F, t117: F, t4179: F, t6559: F, t229: F, t268: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81595, t81597, t81598, t81600, t81602, t81612) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1765::<F>(t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653, t22641, t2588);
        let (t81613, t81615, t81632, t81633, t81640, t81651) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1766::<F>(t225, t814, t6648, t81612, t22715, t6551, t6640, t117, t4179, t6559, t229, t268);
    (t81595, t81597, t81598, t81600, t81602, t81612, t81613, t81615, t81632, t81633, t81640, t81651)
}
