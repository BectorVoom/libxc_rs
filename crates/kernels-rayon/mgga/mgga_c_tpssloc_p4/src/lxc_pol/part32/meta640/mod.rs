//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta640(t28: f64, t40772: f64, t1649: f64, t2752: f64, t1437: f64, t6509: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t89953, t89992, t90090, t90094, t90098, t90101) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2058(t28, t40772, t1649, t2752, t1437, t6509, t1864, t4021, t1410, t9231, t2240, t3961);
    (t89953, t89992, t90090, t90094, t90098, t90101)
}
