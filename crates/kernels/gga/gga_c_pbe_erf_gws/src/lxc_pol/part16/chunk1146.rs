//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1146/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1146<F: Float>(t22509: F, t4218: F, t14906: F, t4414: F, t1144: F, t14186: F, t859: F, t53334: F, t14945: F, t9270: F, t15022: F, t20154: F, t3067: F, t4216: F, t810: F, t1161: F, t1206: F, t14240: F, t14250: F, t14881: F, t2409: F, t2417: F, t3066: F, t3207: F, t4227: F, t53323: F, t53327: F, t53338: F, t6793: F, t8589: F, t8647: F, t8759: F, t9283: F, t9296: F) -> (F,) {
    let t55059 = t22509 * t4218;
    let t55062 = 7.0 / 72.0 * t4414 * t14906;
    let t55065 = t859 * t1144 * t14186;
    let t55074 = 119.0 / 6912.0 * t53334;
    let t55077 = 7.0 / 72.0 * t9270 * t14945;
    let t55087 = 7.0 / 36.0 * t4414 * t15022;
    let t55090 = t20154 * t3067 * t4216 * t810;
    let t55093 = -t3207 * t2409 * t8589 * t14250 / 16.0 - t3066 * t2409 * t9296 * t4227 * t2417 / 16.0 + 35.0 / 216.0 * t55059 - t55062 - t53323 / 384.0 + t6793 * t55065 / 24.0 - t53327 / 192.0 + t3066 * t2409 * t3067 * t14240 * t1161 / 48.0 - t55074 + t53338 / 768.0 - t55077 - t3207 * t9283 * t1206 * t8759 / 16.0 - t3066 * t9283 * t14881 * t8647 / 8.0 + t55087 - t6793 * t55090 / 12.0;
    (t55093,)
}
