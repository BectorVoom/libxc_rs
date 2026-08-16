//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta526<F: Float>(t28042: F, t508: F, t651: F, t1843: F, t7002: F, t2322: F, t7742: F, t4254: F, t1310: F, t7741: F, t22496: F, t8717: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28043, t28045, t28056, t28058, t28060, t28062, t28063, t28065, t28067) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1876::<F>(t28042, t508, t651, t1843, t7002, t2322, t7742, t4254, t1310, t7741, t22496, t8717);
    (t28043, t28045, t28056, t28058, t28060, t28062, t28063, t28065, t28067)
}
