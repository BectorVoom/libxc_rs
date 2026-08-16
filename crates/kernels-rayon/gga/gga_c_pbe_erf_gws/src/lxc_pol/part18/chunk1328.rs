//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1328/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1328(t3816: f64, t51371: f64, t1125: f64, t54101: f64, t11991: f64, t14011: f64, t54129: f64, t54136: f64, t54153: f64, t54167: f64, t56978: f64, t56980: f64, t56982: f64, t56984: f64, t56986: f64, t56988: f64) -> f64 {
    let t56990 = t51371 * t3816;
    let t56992 = t1125 * t54101;
    let t56994 = t14011 * t11991;
    let t56996 = -t54129 + t56978 / 96.0_f64 + t56980 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t56982 + 5.0_f64 / 384.0_f64 * t56984 - t54136 + t54153 + t56986 / 768.0_f64 - 7.0_f64 / 1152.0_f64 * t56988 + t54167 - 7.0_f64 / 144.0_f64 * t56990 + 7.0_f64 / 72.0_f64 * t56992 + t56994 / 96.0_f64;
    t56996
}
