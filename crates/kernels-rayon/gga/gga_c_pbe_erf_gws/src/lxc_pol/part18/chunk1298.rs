//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1298/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1298(t11530: f64, t50998: f64, t51066: f64, t15309: f64, t51963: f64, t4127: f64, t8751: f64, t13972: f64, t15169: f64, t1193: f64, t14404: f64, t14792: f64, t29775: f64, t3028: f64, t3066: f64, t353: f64, t35566: f64, t51509: f64, t53034: f64, t53042: f64, t53047: f64, t56520: f64, t56525: f64, t56534: f64, t56545: f64, t56548: f64, t859: f64, t8629: f64, t8793: f64) -> f64 {
    let t56551 = t50998 * t51066 * t11530;
    let t56553 = t51963 * t15309;
    let t56555 = t4127 * t8751;
    let t56560 = t13972 * t15169;
    let t56563 = t56520 / 1536.0_f64 + t29775 * t14404 / 24.0_f64 - t56525 / 1536.0_f64 + t8793 * t53034 / 24.0_f64 + t8793 * t53042 / 24.0_f64 + t8793 * t53047 / 24.0_f64 - t56534 / 768.0_f64 + t8629 * t859 * t353 * t1193 * t3028 / 96.0_f64 - t56545 / 384.0_f64 - t56548 / 768.0_f64 + t56551 / 192.0_f64 - 35.0_f64 / 1152.0_f64 * t56553 + t56555 / 48.0_f64 - t3066 * t35566 * t14792 / 8.0_f64 - 7.0_f64 / 2304.0_f64 * t56560 - 119.0_f64 / 13824.0_f64 * t51509;
    t56563
}
