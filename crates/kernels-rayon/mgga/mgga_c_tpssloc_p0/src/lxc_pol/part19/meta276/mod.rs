//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta276(t2663: f64, t3814: f64, t3681: f64, t67: f64, t758: f64, t1294: f64, t9905: f64, t9892: f64, t3826: f64, t588: f64, t3684: f64, t9467: f64, t118: f64, t1284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12098, t12099, t12101, t12103, t12105, t12107, t12109) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1039(t2663, t3814, t3681, t67, t758, t1294, t9905, t9892, t3826, t588, t3684, t9467);
        let t12110 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1040(t118, t1284);
    (t12098, t12099, t12101, t12103, t12105, t12107, t12109, t12110)
}
