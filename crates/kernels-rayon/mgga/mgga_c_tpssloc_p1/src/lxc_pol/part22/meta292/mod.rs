//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1450;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta292(t13563: f64, t13566: f64, t4348: f64, t690: f64, t2815: f64, t4370: f64, t2798: f64, t10595: f64, t1547: f64, t10599: f64, t1553: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1450(t13563, t13566, t4348, t690);
        let (t13603, t13623, t13629, t13634, t13637, t13642) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1451(t13602, t2815, t4370, t2798, t10595, t1547, t10599, t1553, t2403);
    (t13600, t13601, t13602, t13603, t13623, t13629, t13634, t13637, t13642)
}
