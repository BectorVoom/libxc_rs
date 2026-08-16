//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta557<F: Float>(t26024: F, t5661: F, t14054: F, t25986: F, t2661: F, t14046: F, t14050: F, t13850: F, t2482: F, t25981: F, t814: F, t13829: F, t94550: F) -> (F, F, F, F, F, F) {
        let (t98226, t98229, t98235, t98238, t98243, t98258) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1876::<F>(t26024, t5661, t14054, t25986, t2661, t14046, t14050, t13850, t2482, t25981, t814, t13829, t94550);
    (t98226, t98229, t98235, t98238, t98243, t98258)
}
