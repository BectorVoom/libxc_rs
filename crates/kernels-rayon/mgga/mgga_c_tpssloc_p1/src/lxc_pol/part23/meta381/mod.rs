//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta381(t1020: f64, t1616: f64, t248: f64, t43216: f64, t10882: f64, t48569: f64, t10875: f64, t1606: f64, t2402: f64, t973: f64, t1654: f64, t9698: f64) -> (f64, f64, f64, f64, f64) {
        let (t50181, t50193, t50265, t50425, t50834) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1183(t1020, t1616, t248, t43216, t10882, t48569, t10875, t1606, t2402, t973, t1654, t9698);
    (t50181, t50193, t50265, t50425, t50834)
}
