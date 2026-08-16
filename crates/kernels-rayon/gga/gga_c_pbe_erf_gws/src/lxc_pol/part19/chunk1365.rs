//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1365/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1365(t1109: f64, t14918: f64, t15036: f64, t15535: f64, t15537: f64, t22379: f64, t2408: f64, t26604: f64, t3212: f64, t353: f64, t4111: f64, t54962: f64, t55151: f64, t55154: f64, t55238: f64, t55243: f64, t55284: f64, t55698: f64, t56582: f64, t56586: f64, t56588: f64, t56590: f64, t859: f64, t8629: f64, t8654: f64, t8793: f64, t892: f64, t9283: f64) -> f64 {
    let t58257 = t56582 / 384.0_f64 - t56586 / 192.0_f64 - t2408 * t9283 * t55151 * t3212 / 12.0_f64 - t55238 + t56588 / 48.0_f64 + t56590 / 48.0_f64 + 35.0_f64 / 108.0_f64 * t55243 - t8654 * t14918 / 48.0_f64 + t22379 * t15036 / 24.0_f64 + t8629 * t55698 / 48.0_f64 + t8793 * t54962 / 24.0_f64 + t8793 * t55284 / 24.0_f64 + t8793 * t55154 / 24.0_f64 + t26604 * t15537 / 96.0_f64 + t8629 * t859 * t892 * t15535 / 96.0_f64 + t8629 * t859 * t353 * t4111 * t1109 / 96.0_f64;
    t58257
}
