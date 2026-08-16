//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1227/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1227(t1307: f64, t16681: f64, t5709: f64, t3805: f64, t5885: f64, t3797: f64, t5701: f64, t1464: f64, t28338: f64, t94216: f64, t27376: f64, t28392: f64) -> (f64, f64, f64, f64, f64) {
    let t98002 = t5709 * t16681 * t1307;
    let t98006 = t5709 * t5885 * t3805;
    let t98010 = t5701 * t5885 * t3797;
    let t98014 = t1464 * t94216 * t28338;
    let t98016 = t28392 * t27376;
    (t98002, t98006, t98010, t98014, t98016)
}
