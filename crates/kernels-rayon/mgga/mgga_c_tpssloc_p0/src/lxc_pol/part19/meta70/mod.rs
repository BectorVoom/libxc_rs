//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta70 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk435;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk436;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk437;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta70(t2244: f64, t65: f64, t11: f64, t2219: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64, t608: f64, t628: f64, t36: f64, t365: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2245, t2248) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk435(t2244, t65, t11, t2219);
        let t2249 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk436(t2248);
        let t2250 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk437(t25, t28, t2249, zeta_threshold);
        let (t2251, t2252, t2255, t2261, t2262) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk438(t2250, t31, t65, t608, t628, t36, t365, sigma0);
    (t2245, t2248, t2249, t2250, t2251, t2252, t2255, t2261, t2262)
}
