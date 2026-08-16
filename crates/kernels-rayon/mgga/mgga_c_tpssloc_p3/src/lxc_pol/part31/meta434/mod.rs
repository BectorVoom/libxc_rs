//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1568;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta434(t23041: f64, t831: f64, t2627: f64, t59: f64, t240: f64, t812: f64, t2617: f64, t6613: f64, t1878: f64, t244: f64, t2230: f64, t6589: f64, t213: f64, t6593: f64, t229: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23043, t23046, t23047, t23048, t23053, t23056, t23061) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1568(t23041, t831, t2627, t59, t240, t812, t2617, t6613, t1878, t244, t2230, t6589);
        let (t23062, t23063, t23069) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1569(t213, t23061, t6593, t229, t6546);
    (t23043, t23046, t23047, t23048, t23053, t23056, t23061, t23062, t23063, t23069)
}
