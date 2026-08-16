//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 760/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk760<F: Float>(t14117: F, t68455: F, t8842: F, t15208: F, t68922: F, t236: F, t31817: F, t14125: F, t68448: F, t15339: F, t3119: F, t34855: F) -> (F, F, F, F, F) {
    let t73779 = t68455 * t14117 * t8842;
    let t73783 = t68922 * t15208;
    let t73785 = t236 * t31817;
    let t73787 = t68448 * t14125 * t73785;
    let t73790 = t15339 * t34855 * t3119;
    (t73779, t73783, t73785, t73787, t73790)
}
