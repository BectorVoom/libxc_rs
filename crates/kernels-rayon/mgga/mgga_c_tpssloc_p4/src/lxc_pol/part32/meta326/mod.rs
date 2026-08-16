//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta326(t10471: f64, t1209: f64, t11712: f64, t3639: f64, t500: f64, t1285: f64, t2223: f64, t1287: f64, t1291: f64, t9874: f64, t25: f64, t514: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t11914, t11947, t11979, t11981, t11984, t11987) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1357(t10471, t1209, t11712, t3639, t500, t1285, t2223, t1287, t1291, t9874, t25, t514);
    (t11914, t11947, t11979, t11981, t11984, t11987)
}
