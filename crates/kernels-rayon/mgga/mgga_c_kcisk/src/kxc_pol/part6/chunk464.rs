//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 464/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk464(t3805: f64, t472: f64, t300: f64, t967: f64, t425: f64, t1390: f64, t143: f64, t424: f64, t3117: f64, t79: f64, t435: f64, t437: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3806 = t3805 * t472;
    let t3807 = 0.55273148148148148147e-3_f64 * t3806;
    let t3812 = t967 * t300;
    let t3814 = 0.46853067927761790996e-2_f64 * t3812 * t425;
    let t3819 = t143 * t1390;
    let t3830 = t424 * t424;
    let t3831 = 1.0_f64 / t3830;
    let t3841 = t3117 * t79;
    let t3844 = 0.21133333333333333333e-2_f64 * t435 * t3841 * t437;
    (t3806, t3807, t3812, t3814, t3819, t3830, t3831, t3841, t3844)
}
