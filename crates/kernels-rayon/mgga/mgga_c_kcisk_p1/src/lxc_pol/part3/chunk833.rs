//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 833/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk833(t167: f64, t3086: f64, t149: f64, t3085: f64, t143: f64, t12425: f64, t12435: f64, t12499: f64, t12630: f64, t12734: f64, t12786: f64, t151: f64, t175: f64, t213: f64, t2925: f64, t3082: f64, t3087: f64, t3088: f64, t3107: f64, t3125: f64, t60: f64, t852: f64, t945: f64, t955: f64, t972: f64) -> f64 {
    let t12789 = t167 * t3086;
    let t12795 = 1.0_f64 / t3085 / t149;
    let t12796 = t143 * t12795;
    let t12809 = -0.70279601891642686494e-2_f64 * t213 * t151 - 0.14055920378328537299e-1_f64 * t12786 * t955 - 0.21083880567492805948e-1_f64 * t12789 * t3088 + 0.70279601891642686494e-2_f64 * t3082 * t3107 - 0.28111840756657074598e-1_f64 * t12796 * t12435 + 0.21083880567492805948e-1_f64 * t3087 * t12734 - 0.23426533963880895498e-2_f64 * t945 * t12499 - t12630 * t175 - 3.0_f64 * t2925 * t972 - 3.0_f64 * t852 * t3125 - t60 * t12425;
    t12809
}
