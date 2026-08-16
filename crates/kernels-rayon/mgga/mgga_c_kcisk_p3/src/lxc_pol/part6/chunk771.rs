//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 771/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk771(t15762: f64, t233: f64, t222: f64, t3276: f64, t227: f64, t3288: f64, t3180: f64, t3463: f64, t3275: f64, t3188: f64, t2454: f64, t5183: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15763 = t233 * t15762;
    let t15772 = 1.0_f64 / t3276 / t222;
    let t15783 = 1.0_f64 / t3288 / t227;
    let t15799 = 3.0_f64 * t3180;
    let t15800 = 3.0_f64 * t3463;
    let t15803 = 3.0_f64 * t3275;
    let t15804 = 6.0_f64 * t3188;
    let t15858 = t5183 * t2454;
    (t15763, t15772, t15783, t15799, t15800, t15803, t15804, t15858)
}
