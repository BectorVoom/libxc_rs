//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta139(t1023: f64, t884: f64, t3071: f64, t225: f64, t3020: f64, t68: f64, t369: f64, t374: f64, t376: f64, t677: f64, t370: f64, t35: f64, t365: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3072, t3073, t3076, t3077, t3078, t3082, t3084, t3087) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk784(t1023, t884, t3071, t225, t3020, t68, t369, t374, t376, t677, t370, t35, t365, t612);
    (t3072, t3073, t3076, t3077, t3078, t3082, t3084, t3087)
}
