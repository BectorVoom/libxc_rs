//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 947/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk947(t3886: f64, t965: f64, t12925: f64, t1383: f64, t12831: f64, t3661: f64, t12952: f64, t457: f64, t3894: f64, t1384: f64, t3119: f64, t1399: f64, t3123: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14014 = t965 * t3886;
    let t14016 = t1383 * t12925;
    let t14019 = t3661 * t12831;
    let t14022 = t457 * t12952;
    let t14025 = t965 * t3894;
    let t14027 = t3119 * t1384;
    let t14029 = t3123 * t1399;
    (t14014, t14016, t14019, t14022, t14025, t14027, t14029)
}
