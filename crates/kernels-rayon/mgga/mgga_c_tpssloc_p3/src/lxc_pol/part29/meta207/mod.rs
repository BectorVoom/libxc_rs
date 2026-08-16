//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta207(t225: f64, t4552: f64, t68: f64, t369: f64, t1031: f64, t1611: f64, t1036: f64, t1612: f64, t1616: f64, t248: f64, t3101: f64, t1020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4615, t4616, t4617, t4622, t4625, t4630, t4631) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1024(t225, t4552, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101, t1020);
    (t4615, t4616, t4617, t4622, t4625, t4630, t4631)
}
