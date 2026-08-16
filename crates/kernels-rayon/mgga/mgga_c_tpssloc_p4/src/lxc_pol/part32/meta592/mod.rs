//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta592(t5584: f64, t852: f64, t1509: f64, t4265: f64, t1519: f64, t4233: f64, t16752: f64, t252: f64, t5527: f64, t828: f64, t5611: f64, t9975: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58166, t58204, t58226, t58262, t58557, t58569, t58688, t58853) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1980(t5584, t852, t1509, t4265, t1519, t4233, t16752, t252, t5527, t828, t5611, t9975);
    (t58166, t58204, t58226, t58262, t58557, t58569, t58688, t58853)
}
