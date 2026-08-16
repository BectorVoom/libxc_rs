//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 842/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk842<F: Float>(t1971: F, t75098: F, t14258: F, t3148: F, t9221: F, t69808: F, t14125: F, t14131: F, t9158: F, t15379: F, t70337: F, t69574: F) -> (F, F, F, F, F) {
    let t75099 = t1971 * t75098;
    let t75100 = t14258 * t75099;
    let t75102 = t9221 * t3148;
    let t75103 = t75102 * t69808;
    let t75106 = t14131 * t14125 * t9158;
    let t75108 = t15379 * t70337;
    let t75110 = F::cast_from(0.23948483403727617128e0_f64) * t69574;
    (t75100, t75103, t75106, t75108, t75110)
}
