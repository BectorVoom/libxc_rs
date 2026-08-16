//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta523(t26223: f64, t26364: f64, t26485: f64, t26500: f64, t533: f64, t1390: f64, t1983: f64, t16521: f64, t1873: f64, t16524: f64, t7015: f64, t5371: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26502, t26503, t26504, t26505, t26533, t26535, t26537) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1857(t26223, t26364, t26485, t26500, t533, t1390, t1983, t16521, t1873, t16524, t7015, t5371, t6534);
    (t26502, t26503, t26504, t26505, t26533, t26535, t26537)
}
