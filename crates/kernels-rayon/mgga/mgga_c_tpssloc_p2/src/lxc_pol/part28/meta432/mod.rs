//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta432(t23083: f64, t6606: f64, t2679: f64, t815: f64, t6605: f64, t2684: f64, t1891: f64, t22822: f64, t133: f64, t6601: f64, t6590: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23084, t23086, t23087, t23089, t23090, t23093, t23095, t23097) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1610(t23083, t6606, t2679, t815, t6605, t2684, t1891, t22822, t133, t6601, t6590, t6604);
    (t23084, t23086, t23087, t23089, t23090, t23093, t23095, t23097)
}
