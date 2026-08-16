//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1372/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1372(t14911: f64, t3083: f64, t15500: f64, t4414: f64, t14918: f64, t3040: f64, t4083: f64, t55375: f64, t55722: f64, t56815: f64, t56818: f64, t56821: f64, t56836: f64, t56840: f64, t56843: f64, t56847: f64, t56849: f64, t56853: f64, t8793: f64, t9958: f64) -> f64 {
    let t58449 = t3083 * t14911;
    let t58457 = t4414 * t15500;
    let t58465 = t56815 / 4.0_f64 - t9958 * t4083 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t58449 - t3040 * t14918 / 48.0_f64 + t8793 * t55722 / 24.0_f64 + t56818 / 96.0_f64 + t56821 / 96.0_f64 - 7.0_f64 / 72.0_f64 * t58457 - 5.0_f64 / 64.0_f64 * t56836 - t56840 / 256.0_f64 - t56843 / 24.0_f64 - t56847 / 384.0_f64 - t56849 / 48.0_f64 + t56853 / 192.0_f64 - t55375;
    t58465
}
