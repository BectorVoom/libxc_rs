//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1958;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta579(t2140: f64, t6169: f64, t1748: f64, t27611: f64, t27617: f64, t27622: f64, t27629: f64, t27684: f64, t27711: f64, t29585: f64, t29594: f64, t29597: f64, t29601: f64, t467: f64, t488: f64, t7326: f64, t8040: f64, t460: f64, t6144: f64, t7320: f64, t6138: f64, t2134: f64, t24729: f64, t24733: f64, t24741: f64, t27604: f64, t27626: f64, t27651: f64, t6192: f64, t6221: f64, t6227: f64, t6232: f64, t7339: f64, t8028: f64, t8031: f64, t8035: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29606, t29610) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1958(t2140, t6169, t1748, t27611, t27617, t27622, t27629, t27684, t27711, t29585, t29594, t29597, t29601, t467, t488, t7326, t8040);
        let (t29614, t29615, t29624, t29625, t29636) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1959(t460, t6144, t7320, t6138, t1748, t2134, t24729, t24733, t24741, t27604, t27626, t27651, t6192, t6221, t6227, t6232, t7339, t8028, t8031, t8035);
    (t29606, t29610, t29614, t29615, t29624, t29625, t29636)
}
