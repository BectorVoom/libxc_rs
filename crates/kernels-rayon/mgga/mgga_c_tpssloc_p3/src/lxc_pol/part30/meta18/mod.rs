//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta18 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk132;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk133;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk134;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk135;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk136;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk137;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta18(t273: f64, t276: f64, t279: f64, t285: f64, t315: f64, t293: f64, t300: f64, t302: f64, t311: f64, t194: f64, t241: f64, zeta_threshold: f64, t131: f64, t39: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t320, t323, t324) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk132(t273, t276, t279, t285);
        let (t328, t330, t334, t335) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk133(t315, t324, t293, t300, t302, t311, t194, t241, zeta_threshold);
        let t336 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk134(t334, t335);
        let t337 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk135(t335);
        let t338 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk136(t131, t337);
        let t339 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk137(t338, t39);
        let t340 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk138(t271);
    (t320, t323, t324, t328, t330, t334, t335, t336, t337, t338, t339, t340)
}
