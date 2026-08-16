//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta110 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk586;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta110(t1512: f64, t2639: f64, t157: f64, t2658: f64, t1409: f64, t184: f64, t1474: f64, t172: f64, t763: f64, t1471: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4187, t4194, t4195, t4199) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk586(t1512, t2639, t157, t2658, t1409, t184, t1474, t172);
        let (t4200, t4205) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk587(t4199, t763, t1471, t706);
    (t4187, t4194, t4195, t4199, t4200, t4205)
}
