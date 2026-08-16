//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1625;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta467(t4017: f64, t71: f64, t12568: f64, t33: f64, t3953: f64, t608: f64, t1437: f64, t641: f64, t72: f64, t4021: f64, t79: f64, t1410: f64, t2235: f64, t3961: f64, t605: f64, t3967: f64, t1433: f64, t645: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26024, t26028, t26055, t26063, t26067, t26070) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1625(t4017, t71, t12568, t33, t3953, t608, t1437, t641, t72, t4021, t79, t1410, t2235);
        let (t26073, t26076, t26090, t26114) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1626(t3961, t605, t3967, t1433, t645, t72, t1458, t649);
    (t26024, t26028, t26055, t26063, t26067, t26070, t26073, t26076, t26090, t26114)
}
