//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1012/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1012<F: Float>(t11683: F, t22796: F, t10760: F, t25684: F, t6535: F, t20305: F, t24161: F, t25466: F, t39506: F, t39509: F, t39512: F, t39514: F, t39517: F, t39520: F, t39523: F, t39524: F) -> (F,) {
    let t39526 = t22796 * t11683;
    let t39529 = t6535 * t10760 * t25684;
    let t39532 = t20305 * t10760 * t24161;
    let t39535 = t6535 * t10760 * t25466;
    let t39537 = 0.32927245914677557994e0 * t39506 + 0.16463622957338778997e0 * t39509 + t39512 - 0.27439371595564631661e-1 * t39514 + 0.21831846657716620896e-2 * t39517 + 0.26198215989259945076e-1 * t39520 + t39523 + 0.5200933044032561138e0 * t39524 - 0.87327386630866483584e-2 * t39526 - 0.87327386630866483584e-2 * t39529 + 0.13099107994629972538e-1 * t39532 - 0.13099107994629972538e-1 * t39535;
    (t39537,)
}
