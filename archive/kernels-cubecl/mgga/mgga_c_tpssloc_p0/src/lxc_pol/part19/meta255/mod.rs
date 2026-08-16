//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1000;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta255<F: Float>(t11620: F, t1246: F, t1235: F, t3507: F, t3625: F, t1155: F, t3375: F, t3396: F, t1164: F, t11128: F, t11133: F, t11179: F, t11182: F, t11184: F, t11187: F, t11405: F, t11409: F, t11426: F, t11429: F, t3395: F, t3400: F, t4883: F, t11194: F, t11272: F, t11280: F, t11288: F, t11290: F, t11296: F, t11472: F, t11475: F, t11480: F, t11482: F, t11484: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11621, t11624, t11625, t11628, t11629, t11631, t11632) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1000::<F>(t11620, t1246, t1235, t3507, t3625, t1155, t3375, t3396, t1164, t11128, t11133, t11179, t11182, t11184, t11187, t11405, t11409, t11426, t11429);
        let (t11634, t11636, t11637) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1001::<F>(t3395, t3400, t4883, t1164, t11194, t11272, t11280, t11288, t11290, t11296, t11472, t11475, t11480, t11482, t11484);
    (t11621, t11624, t11625, t11628, t11629, t11631, t11632, t11634, t11636, t11637)
}
