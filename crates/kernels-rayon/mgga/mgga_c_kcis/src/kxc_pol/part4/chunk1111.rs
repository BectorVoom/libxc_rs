//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1111/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1111(t4723: f64, t9655: f64, t4719: f64, t949: f64, t2938: f64, t13866: f64, t13869: f64, t13871: f64, t13874: f64, t13876: f64, t13878: f64, t13956: f64, t13974: f64, t13977: f64, t14028: f64, t14035: f64, t14038: f64, t14042: f64, t14044: f64, t3035: f64, t45: f64, t4735: f64, t960: f64) -> (f64, f64, f64) {
    let t14046 = 0.32163648644302209644e2_f64 * t9655 * t4723;
    let t14047 = t4719 * t949;
    let t14049 = 4.0_f64 * t2938 * t14047;
    let t14050 = -t13866 + t13869 + t13871 + t13874 + t13876 + t13878 + t13956 + 0.19751789702565206229e-1_f64 * t45 * t13974 + 0.11696446794910408142e1_f64 * t960 * t13977 - 0.58482233974552040708e0_f64 * t960 * t14028 - 0.17315755899375863299e2_f64 * t4735 * t3035 + t14035 + t14038 + t14042 - t14044 + t14046 - t14049;
    (t14046, t14049, t14050)
}
