//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta235(t491: f64, t6150: f64, t1720: f64, t1751: f64, t1730: f64, t1743: f64, t1417: f64, t47: f64, t480: f64, t479: f64, t471: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t6151, t6153, t6158, t6163, t6164, t6165, t6168) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1063(t491, t6150, t1720, t1751, t1730, t1743, t1417, t47, t480, t479, t471, t225);
    (t6151, t6153, t6158, t6163, t6164, t6165, t6168)
}
