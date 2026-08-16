//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta418(t2439: f64, t3421: f64, t12278: f64, t698: f64, t12274: f64, t12256: f64, t39443: f64, t141: f64, t3417: f64, t12268: f64, t1145: f64, t1121: f64, t39457: f64, t12327: f64, t3391: f64, t3399: f64, t12322: f64, t12343: f64, t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43783, t43785, t43787, t43789, t43791, t43793, t43795, t43797) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1560(t2439, t3421, t12278, t698, t12274, t12256, t39443, t141, t3417, t12268, t1145, t1121, t39457);
        let (t43799, t43802, t43804, t43806) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561(t1145, t141, t43797, t12327, t3391, t3399, t12322, t12343, t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795);
    (t43783, t43785, t43787, t43789, t43791, t43793, t43795, t43797, t43799, t43802, t43804, t43806)
}
