//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta348(t1041: f64, t13950: f64, t3114: f64, t4630: f64, t248: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64, t122: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13952, t13959, t13961, t13963, t13965, t13966, t13969) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1388(t1041, t13950, t3114, t4630, t248, t3101, t4650, t1020, t10508, t1616, t122, t247);
    (t13952, t13959, t13961, t13963, t13965, t13966, t13969)
}
