//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1769;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta474(t6997: f64, t7685: f64, t1390: f64, t5187: f64, t6878: f64, t1983: f64, t192: f64, t531: f64, t1982: f64, t5308: f64, t8945: f64, t111: f64, t7450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24989, t24990, t24991, t24993, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1769(t6997, t7685, t1390, t5187, t6878, t1983, t192, t531, t1982);
        let (t24996, t24998, t24999) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1770(t5308, t8945, t24995, t111, t7450);
    (t24989, t24990, t24991, t24993, t24994, t24995, t24996, t24998, t24999)
}
