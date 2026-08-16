//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk875;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta204(t2393: f64, t374: f64, t376: f64, t370: f64, t10250: f64, t977: f64, t3158: f64, t964: f64, t10335: f64, t221: f64, t339: f64, t2955: f64, t995: f64, t3069: f64, t3180: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10375, t10377, t10378, t10381, t10383, t10385, t10388) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk875(t2393, t374, t376, t370, t10250, t977, t3158, t964, t10335, t221, t339, t2955, t995);
        let t10390 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk876(t3069, t3180);
    (t10375, t10377, t10378, t10381, t10383, t10385, t10388, t10390)
}
