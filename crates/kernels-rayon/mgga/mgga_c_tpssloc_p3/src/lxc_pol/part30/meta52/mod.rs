//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk357;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk358;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk359;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk360;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta52(t1003: f64, t68: f64, t369: f64, t191: f64, t349: f64, t361: f64, t363: f64, t336: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1004, t1005, t1008, t1009) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk357(t1003, t68, t369, t191);
        let (t1010, t1011) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk358(t1009, t349, t68);
        let (t1012, t1013, t1014) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk359(t1010, t1011, t361);
        let t1015 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk360(t1014, t363);
        let (t1016, t1017) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk361(t336, t371);
    (t1004, t1005, t1008, t1009, t1010, t1011, t1012, t1013, t1014, t1015, t1016, t1017)
}
