//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 999/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk999<F: Float>(t14047: F, t2938: F, t13866: F, t13869: F, t13871: F, t13874: F, t13876: F, t13878: F, t13956: F, t13974: F, t13977: F, t14028: F, t14035: F, t14038: F, t14042: F, t14044: F, t14046: F, t3035: F, t45: F, t4735: F, t960: F) -> (F, F) {
    let t14049 = 4.0 * t2938 * t14047;
    let t14050 = -t13866 + t13869 + t13871 + t13874 + t13876 + t13878 + t13956 + 0.19751789702565206229e-1 * t45 * t13974 + 0.11696446794910408142e1 * t960 * t13977 - 0.58482233974552040708e0 * t960 * t14028 - 0.17315755899375863299e2 * t4735 * t3035 + t14035 + t14038 + t14042 - t14044 + t14046 - t14049;
    (t14049, t14050)
}
