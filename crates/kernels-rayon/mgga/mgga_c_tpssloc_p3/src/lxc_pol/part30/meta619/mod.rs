//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta619(t22573: f64, t6875: f64, t111: f64, t7002: f64, t26555: f64, t576: f64, t1858: f64, t2029: f64, t5363: f64, t1851: f64, t7020: f64, t1453: f64, t81439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t83886, t83980, t86565, t86567, t86571, t86579, t86586) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2018(t22573, t6875, t111, t7002, t26555, t576, t1858, t2029, t5363, t1851, t7020, t1453, t81439);
    (t83886, t83980, t86565, t86567, t86571, t86579, t86586)
}
