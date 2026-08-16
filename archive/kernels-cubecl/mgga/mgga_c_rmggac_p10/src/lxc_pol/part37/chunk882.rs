//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 882/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk882<F: Float>(t14125: F, t21708: F, t9110: F, t15211: F, t68528: F, t21709: F, t68448: F, t74312: F, t68455: F, t9117: F, t21719: F, t9188: F, t9193: F) -> (F, F, F, F, F) {
    let t75806 = t21708 * t14125 * t9110;
    let t75808 = t68528 * t15211;
    let t75811 = t68448 * t21709 * t74312;
    let t75814 = t68455 * t21709 * t9117;
    let t75818 = t21719 * t9188 * t9193;
    (t75806, t75808, t75811, t75814, t75818)
}
