//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1205/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1205(t1233: f64, t13807: f64, t13869: f64, t13871: f64, t13874: f64, t13876: f64, t13878: f64, t13956: f64, t14035: f64, t14038: f64, t14042: f64, t14044: f64, t14046: f64, t14049: f64, t15296: f64, t15326: f64, t15367: f64, t15463: f64, t15464: f64, t187: f64, t3027: f64, t3600: f64, t4741: f64, t4765: f64, t5261: f64, t972: f64) -> f64 {
    let t15468 = t13869 + t13871 + t13874 + t13876 + t13878 + t13956 - 0.34631511798751726598e2_f64 * t1233 * t13807 - 0.34631511798751726598e2_f64 * t3600 * t4765 - 0.11696446794910408142e1_f64 * t15296 * t972 - 0.58482233974552040708e0_f64 * t5261 * t3027 + 0.23392893589820816284e1_f64 * t3600 * t4741 + t14035 + t14038 + t14042 - t14044 + t14046 - t14049 + t187 * (t15326 + t15367 + t15463 + t15464);
    t15468
}
