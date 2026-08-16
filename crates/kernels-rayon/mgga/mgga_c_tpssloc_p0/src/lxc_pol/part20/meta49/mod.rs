//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta49 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk357;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk358;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk359;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk360;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk361;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk362;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta49(t134: f64, t340: f64, t344: f64, t221: f64, t339: f64, t209: f64, t338: f64, t39: f64, t119: f64, t60: f64, t270: f64, t271: f64, t883: f64, t607: f64, t906: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t967, t969, t971, t972) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk357(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk358(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk359(t119, t60);
        let t976 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk360(t270, t271);
        let t977 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk361(t974, t976);
        let t978 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk362(t344, t883);
        let (t979, t980, t984) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk363(t607, t978, t977, t906, t910);
    (t967, t969, t971, t972, t973, t974, t976, t977, t978, t979, t980, t984)
}
