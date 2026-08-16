//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1753;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1754;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta465(t491: f64, t7319: f64, t3439: f64, t461: f64, t225: f64, t1089: f64, t1240: f64, t3597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24590, t24594, t24595, t24600, t24601) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1753(t491, t7319, t3439, t461, t225);
        let t24602 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1754(t1089, t1240);
        let t24615 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1755(t225, t3597);
    (t24590, t24594, t24595, t24600, t24601, t24602, t24615)
}
