//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1959;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta619<F: Float>(t2018: F, t22125: F, t807: F, t6864: F, t94455: F, t26024: F, t6846: F, t22061: F, t25986: F, t2661: F, t22026: F, t94550: F, t22052: F, t7271: F, t22056: F, t25972: F, t27932: F, t74477: F, t74419: F, t98196: F, t74423: F, t22021: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t108587, t108590, t108592, t108601, t108604) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1959::<F>(t2018, t22125, t807, t6864, t94455, t26024, t6846, t22061, t25986, t2661, t22026, t94550);
        let (t108606, t108608, t108615, t108617, t108619, t108623) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1960::<F>(t22052, t7271, t22056, t25972, t27932, t74477, t74419, t98196, t74423, t22021, t25986, t2661);
    (t108587, t108590, t108592, t108601, t108604, t108606, t108608, t108615, t108617, t108619, t108623)
}
