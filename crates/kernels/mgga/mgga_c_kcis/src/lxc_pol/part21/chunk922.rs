//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 922/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk922<F: Float>(t13866: F, t13869: F, t13871: F, t13874: F, t13876: F, t13878: F, t13956: F, t14035: F, t14038: F, t14042: F, t14044: F, t14046: F, t14049: F, t1233: F, t13807: F, t15296: F, t15326: F, t15367: F, t15463: F, t187: F, t3027: F, t3600: F, t4741: F, t4765: F, t5261: F, t972: F) -> (F,) {
    let t15464 = t13866 - t13869 - t13871 - t13874 - t13876 - t13878 - t13956 - t14035 - t14038 - t14042 + t14044 - t14046 + t14049;
    let t15468 = t13869 + t13871 + t13874 + t13876 + t13878 + t13956 - 0.34631511798751726598e2 * t1233 * t13807 - 0.34631511798751726598e2 * t3600 * t4765 - 0.11696446794910408142e1 * t15296 * t972 - 0.58482233974552040708e0 * t5261 * t3027 + 0.23392893589820816284e1 * t3600 * t4741 + t14035 + t14038 + t14042 - t14044 + t14046 - t14049 + t187 * (t15326 + t15367 + t15463 + t15464);
    (t15468,)
}
