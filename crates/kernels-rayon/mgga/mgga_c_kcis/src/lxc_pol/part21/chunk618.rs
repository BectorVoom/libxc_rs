//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 618/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk618(t4893: f64, t4919: f64, t1030: f64, t1083: f64, t1697: f64, t1745: f64, t278: f64, t305: f64, t3056: f64, t3057: f64, t3059: f64, t3061: f64, t3158: f64, t339: f64, t4768: f64, t4831: f64, t4833: f64, t4837: f64, t4840: f64, t4843: f64, t4845: f64, t4849: f64, t4852: f64, t975: f64) -> (f64, f64) {
    let t4920 = t4893 + t4919;
    let t4922 = t3056 + 0.23426533963880895498e-2_f64 * t3057 + 0.46853067927761790996e-2_f64 * t3059 + 0.23426533963880895498e-2_f64 * t4831 + 0.46853067927761790996e-2_f64 * t3061 * t4833 + 0.46853067927761790996e-2_f64 * t1030 * t4837 - 0.46853067927761790996e-2_f64 * t3158 * t4840 + 0.46853067927761790996e-2_f64 * t4843 + 0.46853067927761790996e-2_f64 * t1030 * t4845 + 0.14055920378328537299e-1_f64 * t305 * t4849 - 0.46853067927761790996e-2_f64 * t305 * t4852 - t4768 * t339 - t1697 * t1083 - t975 * t1745 - t278 * t4920;
    (t4920, t4922)
}
