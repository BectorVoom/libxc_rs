//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1973/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1973<F: Float>(t102378: F, t102386: F, t102396: F, t102397: F, t102404: F, t1882: F, t2103: F, t25930: F, t25933: F, t26304: F, t26305: F, t26371: F, t27837: F, t27868: F, t49393: F, t96401: F, t96403: F, t96410: F, t96412: F, t96423: F, t97737: F, t97933: F, t98053: F) -> F {
    let t102406 = -F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26304 * t97737 - F::cast_from(0.17135234354032049604e-1_f64) * t102378 + t96401 + F::cast_from(0.23131639038696784278e-2_f64) * t96403 - F::cast_from(0.4336814094102599731e0_f64) * t98053 * t2103 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t26305 + F::cast_from(0.22849835011101738147e-2_f64) * t102386 - F::cast_from(0.14634331517634470219e-1_f64) * t96410 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26304 * t1882 * t25933 + F::cast_from(0.34270468708064099208e-1_f64) * t96412 - t102396 + F::cast_from(0.26020884564615598386e1_f64) * t27868 * t102397 * t49393 - F::cast_from(0.26020884564615598386e1_f64) * t27837 * t26371 - t102404 - F::cast_from(0.9757440539382783019e-2_f64) * t96423;
    t102406
}
