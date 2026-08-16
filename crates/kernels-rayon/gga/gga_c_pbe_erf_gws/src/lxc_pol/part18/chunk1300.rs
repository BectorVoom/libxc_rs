//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1300/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1300(t15161: f64, t2397: f64, t12074: f64, t3079: f64, t14135: f64, t3912: f64, t51913: f64, t11505: f64, t3972: f64, t3975: f64, t15288: f64, t14757: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t3886: f64, t4052: f64, t53546: f64, t53578: f64, t53584: f64, t53585: f64, t53598: f64, t56578: f64, t56582: f64, t56586: f64, t8589: f64) -> f64 {
    let t56588 = t15161 * t2397;
    let t56590 = t12074 * t3079;
    let t56593 = t3912 * t14135 * t51913;
    let t56596 = t3972 * t3975 * t11505;
    let t56599 = t15288 * t2397;
    let t56601 = t3066 * t2409 * t3067 * t4052 * t3886 / 48.0_f64 + t2408 * t2409 * t8589 * t14757 / 24.0_f64 + t56578 / 96.0_f64 + t56582 / 768.0_f64 - t56586 / 384.0_f64 - t53546 + t56588 / 96.0_f64 + t56590 / 96.0_f64 + t56593 / 48.0_f64 - t53578 + t56596 / 1536.0_f64 - t53584 + 35.0_f64 / 108.0_f64 * t53585 - t53598 + t56599 / 96.0_f64;
    t56601
}
