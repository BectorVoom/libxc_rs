//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1095/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1095(t3316: f64, t5047: f64, t7748: f64, t3424: f64, t377: f64, t3362: f64, t374: f64, t982: f64, t7755: f64, t1096: f64, t3432: f64, t386: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26917 = t5047 * t3316;
    let t26918 = t7748 * t26917;
    let t26920 = t3424 * t377;
    let t26922 = t374 * t3362;
    let t26924 = t374 * t982;
    let t26925 = t26924 * t7755;
    let t26927 = t1096 * t3432;
    let t26929 = sigma0 * t386;
    (t26917, t26918, t26920, t26922, t26924, t26925, t26927, t26929)
}
