//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1134;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta326<F: Float>(t187: F, t22789: F, t13621: F, t13630: F, t13633: F, t22764: F, t22765: F, t22766: F, t22768: F, t22791: F, t9394: F, t9396: F, t9409: F, t9412: F, t13652: F, t13654: F, t9415: F, t9421: F, t9427: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F) -> (F, F, F, F, F, F, F, F) {
        let (t22919, t22920, t22921, t22922, t22923) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1134::<F>(t187, t22789, t13621, t13630, t13633, t22764, t22765, t22766, t22768, t22791, t9394, t9396, t9409, t9412);
        let (t22925, t22926, t22927) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1135::<F>(t13652, t13654, t9415, t9421, t9427, t9514, t9517, t9521, t9546, t9569, t9574, t9577);
    (t22919, t22920, t22921, t22922, t22923, t22925, t22926, t22927)
}
