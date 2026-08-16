//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 612/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk612(t875: f64, t9551: f64, t1971: f64, t3351: f64, t2338: f64, t702: f64, t638: f64, t639: f64, t2474: f64, t640: f64, t3219: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15488 = t875 * t9551;
    let t15489 = t1971 * t15488;
    let t15490 = t3351 * t15489;
    let t15491 = 0.85129199786595678796e-5_f64 * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    let t15495 = 0.15243824895787514157e-3_f64 * t15494;
    let t15496 = t640 * t2474;
    let t15498 = t638 * t639 * t15496;
    let t15499 = 0.15243824895787514157e-3_f64 * t15498;
    let t15500 = t8571 * t3219;
    (t15489, t15491, t15492, t15495, t15496, t15499, t15500)
}
