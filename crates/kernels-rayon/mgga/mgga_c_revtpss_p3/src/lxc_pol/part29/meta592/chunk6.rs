//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1973/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1973(t102378: f64, t102386: f64, t102396: f64, t102397: f64, t102404: f64, t1882: f64, t2103: f64, t25930: f64, t25933: f64, t26304: f64, t26305: f64, t26371: f64, t27837: f64, t27868: f64, t49393: f64, t96401: f64, t96403: f64, t96410: f64, t96412: f64, t96423: f64, t97737: f64, t97933: f64, t98053: f64) -> f64 {
    let t102406 = -0.17347256376410398924e1_f64 * t25930 * t26304 * t97737 - 0.17135234354032049604e-1_f64 * t102378 + t96401 + 0.23131639038696784278e-2_f64 * t96403 - 0.4336814094102599731e0_f64 * t98053 * t2103 - 0.17347256376410398924e1_f64 * t97933 * t26305 + 0.22849835011101738147e-2_f64 * t102386 - 0.14634331517634470219e-1_f64 * t96410 - 0.17347256376410398924e1_f64 * t25930 * t26304 * t1882 * t25933 + 0.34270468708064099208e-1_f64 * t96412 - t102396 + 0.26020884564615598386e1_f64 * t27868 * t102397 * t49393 - 0.26020884564615598386e1_f64 * t27837 * t26371 - t102404 - 0.9757440539382783019e-2_f64 * t96423;
    t102406
}
