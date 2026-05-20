//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta444<F: Float>(t1427: F, t1903: F, t22: F, t9647: F, t14296: F, t9303: F, t5718: F, t9292: F, t14099: F, t2453: F, t5603: F, t9692: F, t3915: F, t5721: F, t9288: F, t14293: F, t9664: F, t14103: F, t9285: F, t9674: F, t13726: F, t10115: F, t1900: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47781, t47786, t47802, t47856, t47863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403::<F>(t1427, t1903, t22, t9647, t14296, t9303, t5718, t9292, t14099, t2453, t5603, t9692);
        let (t47904, t47920, t47932, t47938, t47961) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1404::<F>(t3915, t5721, t9288, t14293, t9664, t14103, t9285, t9674, t13726, t9303, t10115, t1900);
    (t47781, t47786, t47802, t47856, t47863, t47904, t47920, t47932, t47938, t47961)
}
