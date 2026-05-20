//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1903;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta528<F: Float>(t2007: F, t4292: F, t670: F, t7883: F, t1843: F, t7002: F, t651: F, t2322: F, t7742: F, t4254: F, t1310: F, t7741: F, t22496: F, t8717: F, t25082: F, t1469: F, t25129: F, t25132: F, t25137: F, t4181: F, t4186: F, t6968: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28050, t28053, t28056, t28058, t28060, t28062, t28063) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1903::<F>(t2007, t4292, t670, t7883, t1843, t7002, t651, t2322, t7742, t4254, t1310, t7741);
        let (t28065, t28067, t28069, t28076) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1904::<F>(t28063, t651, t22496, t8717, t25082, t1469, t25129, t25132, t25137, t4181, t4186, t6968);
    (t28050, t28053, t28056, t28058, t28060, t28062, t28063, t28065, t28067, t28069, t28076)
}
