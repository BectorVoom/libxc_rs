//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta356(t13213: f64, t13268: f64, t13331: f64, t13375: f64, t218: f64, t1509: f64, t852: f64, t829: f64, t252: f64, t4233: f64, t4182: f64, t2684: f64, t4282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13377, t13378, t13380, t13381, t13384, t13385, t13388) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1333(t13213, t13268, t13331, t13375, t218, t1509, t852, t829, t252, t4233, t4182, t2684, t4282);
    (t13377, t13378, t13380, t13381, t13384, t13385, t13388)
}
