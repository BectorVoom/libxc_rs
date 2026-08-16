//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 892/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk892(t3512: f64, t3769: f64, t1339: f64, t3583: f64, t3764: f64, t1340: f64, t3575: f64, t3759: f64, t12952: f64, t1341: f64, t3508: f64, t3743: f64) -> (f64, f64, f64, f64, f64) {
    let t13346 = t3512 * t3769;
    let t13347 = t1339 * t13346;
    let t13349 = t3764 * t3583;
    let t13350 = t1340 * t13349;
    let t13351 = t1339 * t13350;
    let t13353 = t3764 * t3575;
    let t13354 = t1340 * t13353;
    let t13355 = t3759 * t13354;
    let t13357 = t1341 * t12952;
    let t13358 = t1340 * t13357;
    let t13359 = t3759 * t13358;
    let t13361 = t3508 * t3743;
    (t13347, t13351, t13355, t13359, t13361)
}
