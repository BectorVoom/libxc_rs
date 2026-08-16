//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 899/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk899(t3495: f64, t3512: f64, t1411: f64, t3494: f64, t3502: f64, t1340: f64, t3503: f64, t10519: f64, t10520: f64, t12924: f64, t8: f64, t1450: f64) -> (f64, f64, f64, f64, f64) {
    let t13412 = t3512 * t3495;
    let t13413 = t1411 * t13412;
    let t13415 = t3494 * t3502;
    let t13416 = t1340 * t13415;
    let t13417 = t1411 * t13416;
    let t13419 = t3512 * t3503;
    let t13420 = t1411 * t13419;
    let t13423 = t12924 * t8 - t10519 + t10520;
    let t13424 = t1450 * t13423;
    (t13413, t13417, t13420, t13423, t13424)
}
