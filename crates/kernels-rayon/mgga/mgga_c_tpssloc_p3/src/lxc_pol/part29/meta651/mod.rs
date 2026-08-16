//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta651(t5107: f64, t652: f64, t6534: f64, t22574: f64, t56198: f64, t8643: f64, t26162: f64, t57802: f64, t22597: f64, t7685: f64, t2018: f64, t3734: f64, t1983: f64, t7687: f64, t26062: f64, t645: f64, t72: f64, t26066: f64, t2307: f64, t7431: f64, t1437: f64, t6509: f64, t1864: f64, t4021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90051, t90059, t90062, t90064, t90065) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175(t5107, t652, t6534, t22574, t56198, t8643, t26162, t57802, t22597, t7685, t2018, t3734);
        let (t90068, t90072, t90076, t90080, t90090, t90094) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2176(t1983, t7687, t90065, t26062, t645, t72, t26066, t2307, t7431, t1437, t6509, t1864, t4021);
    (t90051, t90059, t90062, t90064, t90068, t90072, t90076, t90080, t90090, t90094)
}
