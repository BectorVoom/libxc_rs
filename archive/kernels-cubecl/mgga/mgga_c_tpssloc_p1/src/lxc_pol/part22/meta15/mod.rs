//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta15 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk120;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk121;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk122;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk123;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk124;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk125;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk126;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta15<F: Float>(t253: F, t259: F, t144: F, t186: F, t189: F, t193: F, t202: F, t118: F, t120: F, t121: F, t154: F, t40: F, t119: F, t133: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t261, t262) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk120::<F>(t253, t259);
        let t265 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk121::<F>(t144, t186, t189, t193, t202, t262);
        let t268 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk122::<F>(t118, t120);
        let (t269, t270, t271) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk123::<F>(t121, t154, t40);
        let t273 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk124::<F>(t268, t269, t271);
        let t275 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk125::<F>(t273);
        let t276 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk126::<F>(t273);
        let (t279, t281) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk127::<F>(t273, t119, t133);
    (t261, t262, t265, t268, t269, t270, t271, t273, t275, t276, t279, t281)
}
