//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1359;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta300(t340: f64, t63: f64, t344: f64, t221: f64, t339: f64, t2960: f64, t2974: f64, t135: f64, t3016: f64, t973: f64, t1036: f64, t3078: f64, t1032: f64, t3082: f64, t2393: f64, t374: f64, t376: f64, t370: f64, t3158: f64, t964: f64, t2955: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10335, t10336, t10339, t10342, t10353, t10370) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1359(t340, t63, t344, t221, t339, t2960, t2974, t135, t3016, t973, t1036, t3078);
        let (t10372, t10375, t10377, t10381, t10383, t10385, t10388) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1360(t1032, t3082, t2393, t374, t376, t370, t3158, t964, t10335, t221, t339, t2955, t995);
    (t10336, t10339, t10342, t10353, t10370, t10372, t10375, t10377, t10381, t10383, t10385, t10388)
}
