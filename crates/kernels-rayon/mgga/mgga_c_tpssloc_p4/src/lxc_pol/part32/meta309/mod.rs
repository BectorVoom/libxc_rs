//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta309(t154: f64, t3061: f64, t10544: f64, t276: f64, t285: f64, t273: f64, t2928: f64, t941: f64, t2931: f64, t323: f64, t2841: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10564, t10577, t10595, t10599, t10608, t10629, t10632, t10636, t10655) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1335(t154, t3061, t10544, t276, t285, t273, t2928, t941, t2931, t323, t2841, t888);
    (t10564, t10577, t10595, t10599, t10608, t10629, t10632, t10636, t10655)
}
