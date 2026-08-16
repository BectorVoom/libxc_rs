//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 683/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk683(t11401: f64, t695: f64, t10459: f64, t707: f64, t10463: f64, t708: f64, t4663: f64, t213: f64, t568: f64, t682: f64, t5100: f64, t680: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11402 = t11401 * t695;
    let t11417 = t10459 * t707;
    let t11418 = t708 * t10463;
    let t11443 = t4663 * t708;
    let t11458 = t213 * t568;
    let t11460 = 0.14055920378328537299e-1_f64 * t11458 * t682;
    let t11480 = 1.0_f64 / t5100 / t680;
    (t11402, t11417, t11418, t11443, t11458, t11460, t11480)
}
