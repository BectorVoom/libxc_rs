//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta560(t13487: f64, t86721: f64, t22960: f64, t58071: f64, t2: f64, t2752: f64, t584: f64, t868: f64, t4303: f64, t606: f64, t870: f64, t776: f64) -> (f64, f64, f64, f64, f64) {
        let (t86722, t86727, t86732, t86746, t86755) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1832(t13487, t86721, t22960, t58071, t2, t2752, t584, t868, t4303, t606, t870, t776);
    (t86722, t86727, t86732, t86746, t86755)
}
