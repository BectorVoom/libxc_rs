//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 890/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk890<F: Float>(t1322: F, t235: F, t29837: F, t15144: F, t352: F, t1326: F, t27: F, t9145: F, t16129: F, t70489: F, t1469: F, t34976: F, t39851: F, t665: F) -> (F, F, F, F, F) {
    let t75961 = t235 * t29837 * t1322;
    let t75962 = t15144 * t352;
    let t75963 = t1326 * t75962;
    let t75964 = t75961 * t75963;
    let t75966 = t27 * t9145;
    let t75968 = t70489 * t16129 * t75966;
    let t75972 = t39851 * t34976 * t665 * t1469;
    (t75962, t75963, t75964, t75968, t75972)
}
