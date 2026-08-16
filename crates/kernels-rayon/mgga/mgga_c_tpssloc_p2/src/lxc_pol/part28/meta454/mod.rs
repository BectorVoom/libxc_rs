//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta454(t23095: f64, t23105: f64, t23107: f64, t23140: f64, t23143: f64, t23100: f64, t23114: f64, t23117: f64, t23119: f64, t23125: f64, t23128: f64, t23130: f64, t23134: f64, t23136: f64, t23147: f64, t24217: f64, t218: f64, t7084: f64, t798: f64, t23013: f64, t23031: f64, t2684: f64, t7101: f64, t2047: f64, t2627: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24218, t24220, t24221, t24230, t24231, t24233) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1651(t23095, t23105, t23107, t23140, t23143, t23100, t23114, t23117, t23119, t23125, t23128, t23130, t23134, t23136, t23147);
        let (t24234, t24235, t24237, t24246, t24250, t24251, t24256) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1652(t24217, t24233, t218, t7084, t798, t23013, t23031, t2684, t7101, t2047, t2627, t2633);
    (t24218, t24220, t24221, t24230, t24231, t24234, t24235, t24237, t24246, t24250, t24251, t24256)
}
