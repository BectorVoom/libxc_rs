//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta551(t19299: f64, t33: f64, t5441: f64, t71: f64, t5389: f64, t79: f64, t72: f64, t1410: f64, t3953: f64, t1433: f64, t1437: f64, t5445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27937, t27956, t27960, t27961, t27966, t27971, t27972, t27975) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1910(t19299, t33, t5441, t71, t5389, t79, t72, t1410, t3953, t1433, t1437, t5445);
    (t27937, t27956, t27960, t27961, t27966, t27971, t27972, t27975)
}
