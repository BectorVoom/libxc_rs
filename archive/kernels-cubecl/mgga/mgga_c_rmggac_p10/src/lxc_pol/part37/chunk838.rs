//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 838/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk838<F: Float>(t13962: F, t3056: F, t8475: F, t13966: F, t2046: F, t8486: F, t13839: F, t1652: F, t2044: F, t3076: F, t15035: F, t2160: F, t638: F) -> (F, F, F, F) {
    let t75074 = t3056 * t13962 * t8475;
    let t75077 = t2046 * t13966 * t8486;
    let t75081 = t13839 * t2044 * t3076 * t1652;
    let t75084 = t638 * t2160 * t15035;
    (t75074, t75077, t75081, t75084)
}
