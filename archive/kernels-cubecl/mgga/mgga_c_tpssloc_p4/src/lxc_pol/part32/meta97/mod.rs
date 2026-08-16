//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk622;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk623;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk624;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk625;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk626;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk627;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta97<F: Float>(t2127: F, t467: F, t480: F, t1932: F, t3: F, t52: F, t225: F, t461: F, t479: F, t477: F, sigma2: F, t483: F, t471: F, t488: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2128, t2130, t2131, t2132) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk622::<F>(t2127, t467, t480, t1932);
        let t2133 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk623::<F>(t3, t52);
        let t2134 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk624::<F>(t2132, t2133);
        let (t2135, t2136) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk625::<F>(t225, t461, t479);
        let t2139 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk626::<F>(t477, sigma2);
        let t2140 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk627::<F>(t2139, t483);
        let (t2141, t2144) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk628::<F>(t2140, t471, t2128, t2134, t2136, t488);
    (t2128, t2130, t2131, t2132, t2133, t2134, t2135, t2136, t2139, t2140, t2141, t2144)
}
