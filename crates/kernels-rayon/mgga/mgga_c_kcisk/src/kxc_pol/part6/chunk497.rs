//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 497/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk497(t568: f64, t967: f64, t682: f64, t143: f64, t1849: f64, t681: f64, t1394: f64, t429: f64, t686: f64, t3841: f64, t435: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5082 = t967 * t568;
    let t5084 = 0.46853067927761790996e-2_f64 * t5082 * t682;
    let t5089 = t143 * t1849;
    let t5100 = t681 * t681;
    let t5101 = 1.0_f64 / t5100;
    let t5122 = 0.8197e-2_f64 * t429 * t1394 * t686;
    let t5125 = 0.21133333333333333333e-2_f64 * t435 * t3841 * t690;
    (t5082, t5084, t5089, t5100, t5101, t5122, t5125)
}
