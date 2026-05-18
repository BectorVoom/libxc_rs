//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 922/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk922<F: Float>(t4723: F, t9655: F, t4719: F, t949: F, t2938: F, t13866: F, t13869: F, t13871: F, t13874: F, t13876: F, t13878: F, t13956: F, t13974: F, t13977: F, t14028: F, t14035: F, t14038: F, t14042: F, t14044: F, t3035: F, t45: F, t4735: F, t960: F) -> (F, F, F) {
    let t14046 = F::new(0.32163648644302209644e2) * t9655 * t4723;
    let t14047 = t4719 * t949;
    let t14049 = F::new(4.0) * t2938 * t14047;
    let t14050 = -t13866 + t13869 + t13871 + t13874 + t13876 + t13878 + t13956 + F::new(0.19751789702565206229e-1) * t45 * t13974 + F::new(0.11696446794910408142e1) * t960 * t13977 - F::new(0.58482233974552040708e0) * t960 * t14028 - F::new(0.17315755899375863299e2) * t4735 * t3035 + t14035 + t14038 + t14042 - t14044 + t14046 - t14049;
    (t14046, t14049, t14050)
}
