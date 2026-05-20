//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1908;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta564<F: Float>(t25978: F, t5629: F, t1885: F, t94459: F, t26024: F, t5661: F, t14054: F, t25986: F, t2661: F, t13874: F, t7271: F, t14046: F, t14050: F, t13850: F, t2482: F, t25981: F, t814: F, t13962: F, t26028: F, t14020: F, t7252: F, t13829: F, t94550: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98222, t98224, t98226, t98229, t98231, t98235) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1908::<F>(t25978, t5629, t1885, t94459, t26024, t5661, t14054, t25986, t2661, t13874, t7271, t14046);
        let (t98238, t98243, t98245, t98253, t98258) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1909::<F>(t14050, t25986, t2661, t13850, t2482, t25981, t814, t13962, t26028, t14020, t7252, t13829, t94550);
    (t98222, t98224, t98226, t98229, t98231, t98235, t98238, t98243, t98245, t98253, t98258)
}
