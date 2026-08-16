//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1754;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta468(t1930: f64, t23508: f64, t6741: f64, t3030: f64, t3127: f64, t363: f64, t1014: f64, t1940: f64, t3046: f64, t354: f64, t3053: f64, t6765: f64, t3037: f64, t3033: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23509, t23510, t23511, t23512, t23518, t23519, t23528, t23529) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1754(t1930, t23508, t6741, t3030, t3127, t363, t1014, t1940, t3046, t354);
        let (t23533, t23535, t23536, t23537) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1755(t3053, t6765, t3127, t3037, t3033, sigma0);
    (t23509, t23510, t23511, t23512, t23518, t23519, t23528, t23529, t23533, t23535, t23536, t23537)
}
