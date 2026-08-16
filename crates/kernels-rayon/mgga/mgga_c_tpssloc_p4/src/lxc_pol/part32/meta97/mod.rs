//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk622;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk623;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk624;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk625;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk626;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk627;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta97(t2127: f64, t467: f64, t480: f64, t1932: f64, t3: f64, t52: f64, t225: f64, t461: f64, t479: f64, t477: f64, sigma2: f64, t483: f64, t471: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2128, t2130, t2131, t2132) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk622(t2127, t467, t480, t1932);
        let t2133 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk623(t3, t52);
        let t2134 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk624(t2132, t2133);
        let (t2135, t2136) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk625(t225, t461, t479);
        let t2139 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk626(t477, sigma2);
        let t2140 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk627(t2139, t483);
        let (t2141, t2144) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk628(t2140, t471, t2128, t2134, t2136, t488);
    (t2128, t2130, t2131, t2132, t2133, t2134, t2135, t2136, t2139, t2140, t2141, t2144)
}
