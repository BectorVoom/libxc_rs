//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 614/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk614(t15224: f64, t15228: f64, t15232: f64, t15236: f64, t875: f64, t9551: f64, t1971: f64, t3351: f64, t2338: f64, t702: f64, t638: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15484 = 0.17519306092901367188e-6_f64 * t15224;
    let t15485 = 0.15961724959986689775e-4_f64 * t15228;
    let t15486 = 0.1276937996798935182e-4_f64 * t15232;
    let t15487 = 0.2553875993597870364e-4_f64 * t15236;
    let t15488 = t875 * t9551;
    let t15489 = t1971 * t15488;
    let t15490 = t3351 * t15489;
    let t15491 = 0.85129199786595678796e-5_f64 * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    (t15484, t15485, t15486, t15487, t15489, t15491, t15492, t15494)
}
