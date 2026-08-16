//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2096;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta546(t13633: f64, t13615: f64, t13620: f64, t13623: f64, t13634: f64, t13635: f64, t22192: f64, t22194: f64, t22196: f64, t22197: f64, t22198: f64, t22199: f64, t22200: f64, t22201: f64, t9394: f64, t9415: f64, t9422: f64, t9559: f64, t9566: f64, t9570: f64, t9578: f64, t13643: f64, t9421: f64, t9427: f64, t9429: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22202, t22203) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2096(t13633, t13615, t13620, t13623, t13634, t13635, t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t9394, t9415);
        let (t22205, t22206, t22207, t22208, t22209, t22210) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2097(t9422, t9559, t9566, t9570, t9578, t13643, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577);
    (t22202, t22203, t22205, t22206, t22207, t22208, t22209, t22210)
}
