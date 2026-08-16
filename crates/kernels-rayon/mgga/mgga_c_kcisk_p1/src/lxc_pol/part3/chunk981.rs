//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 981/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk981(t14111: f64, t416: f64, t140: f64, t1477: f64, t430: f64, t1390: f64, t1402: f64, t1471: f64, t3278: f64, t12830: f64, t4272: f64, t12951: f64, t451: f64) -> (f64, f64, f64, f64, f64) {
    let t14464 = t416 * t14111;
    let t14469 = t140 * t430 * t1477;
    let t14475 = t1402 * t1390;
    let t14477 = t1471 * t14475 * t3278;
    let t14481 = t1471 * t4272 * t12830;
    let t14484 = t451 * t12951;
    (t14464, t14469, t14477, t14481, t14484)
}
