//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1173/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1173(t7058: f64, t95644: f64, t7064: f64, t25387: f64, t95571: f64, t11050: f64, t26497: f64, t92975: f64, t92942: f64, t92944: f64, t92946: f64, t92948: f64, t92952: f64, t92956: f64, t92958: f64, t92960: f64, t92963: f64, t92966: f64, t92969: f64, t92971: f64, t92973: f64) -> (f64, f64, f64, f64, f64) {
    let t95645 = t7058 * t95644;
    let t95647 = t7064 * t95644;
    let t95649 = t25387 * t95571;
    let t95651 = t26497 * t11050;
    let t95666 = 0.18295201011342718161e-3_f64 * t92975;
    let t95667 = 0.51448821741683684367e-2_f64 * t92942 - 0.51448821741683684367e-1_f64 * t92944 + 0.10289764348336736873e-1_f64 * t92946 + 0.10289764348336736873e-1_f64 * t92948 - 0.96037800584476210818e-1_f64 * t92952 + 0.12196800674228478774e-2_f64 * t92956 + 0.10289764348336736873e-1_f64 * t92958 - 0.25724410870841842183e-2_f64 * t92960 + 0.30492001685571196935e-4_f64 * t92963 - 0.2168591159877823526e-3_f64 * t92966 - 35.0_f64 / 36.0_f64 * t92969 + 7.0_f64 / 24.0_f64 * t92971 - t92973 / 24.0_f64 + t95666;
    (t95645, t95647, t95649, t95651, t95667)
}
