//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta418<F: Float>(t2439: F, t3421: F, t12278: F, t698: F, t12274: F, t12256: F, t39443: F, t141: F, t3417: F, t12268: F, t1145: F, t1121: F, t39457: F, t12327: F, t3391: F, t3399: F, t12322: F, t12343: F, t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43783, t43785, t43787, t43789, t43791, t43793, t43795, t43797) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1560::<F>(t2439, t3421, t12278, t698, t12274, t12256, t39443, t141, t3417, t12268, t1145, t1121, t39457);
        let (t43799, t43802, t43804, t43806) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561::<F>(t1145, t141, t43797, t12327, t3391, t3399, t12322, t12343, t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795);
    (t43783, t43785, t43787, t43789, t43791, t43793, t43795, t43797, t43799, t43802, t43804, t43806)
}
