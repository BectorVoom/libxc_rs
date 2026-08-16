//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2154;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta569(t187: f64, t22789: f64, t13621: f64, t13630: f64, t13633: f64, t22764: f64, t22765: f64, t22766: f64, t22768: f64, t22791: f64, t9394: f64, t9396: f64, t9409: f64, t9412: f64, t13652: f64, t13654: f64, t9415: f64, t9421: f64, t9427: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22919, t22920, t22921, t22922, t22923) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2154(t187, t22789, t13621, t13630, t13633, t22764, t22765, t22766, t22768, t22791, t9394, t9396, t9409, t9412);
        let (t22925, t22926, t22927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2155(t13652, t13654, t9415, t9421, t9427, t9514, t9517, t9521, t9546, t9569, t9574, t9577);
    (t22919, t22920, t22921, t22922, t22923, t22925, t22926, t22927)
}
