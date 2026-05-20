//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta310<F: Float>(t13848: F, t6869: F, t9818: F, t9816: F, t1413: F, t6816: F, t547: F, t807: F, t4011: F, t6836: F, t6871: F, t9962: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22102, t22103, t22125, t22126, t22127, t22129, t22130, t22131, t22156) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1097::<F>(t13848, t6869, t9818, t9816, t1413, t6816, t547, t807, t4011, t6836, t6871, t9962);
    (t22102, t22103, t22125, t22126, t22127, t22129, t22130, t22131, t22156)
}
