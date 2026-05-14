//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1192/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1192<F: Float>(t15216: F, t28112: F, t26955: F, t15220: F, t26960: F, t28124: F, t28117: F, t2829: F, t5281: F, t5310: F, t2845: F, t5302: F, t11020: F, t26977: F, t28096: F, t28204: F, t3515: F, t95884: F, t95887: F, t96739: F, t96857: F) -> (F, F, F, F) {
    let t96975 = t15216 * t28112;
    let t96977 = 0.10306077835648148148e-4 * t26955 * t96975;
    let t96980 = 0.10297067901234567901e-3 * t26960 * t15220 * t28124;
    let t96993 = 0.15445601851851851852e-3 * t26960 * t15216 * t28117;
    let t96995 = t5310 * t5281 * t2829;
    let t96999 = t5302 * t5281 * t2845;
    let t97006 = -0.92754700520833333333e-4 * t26955 * t96739 + t96977 - t96980 - 0.46377350260416666666e-4 * t26955 * t96857 + 0.11584201388888888889e-3 * t26960 * t3515 * t28096 * t2829 + 0.15445601851851851852e-3 * t26960 * t11020 * t28096 * t2845 + t96993 + 0.11584201388888888889e-3 * t26960 * t96995 + 0.15445601851851851852e-3 * t26960 * t96999 - 0.11607361111111111111e-2 * t95884 + 0.61905925925925925925e-2 * t95887 - 0.13913205078125e-3 * t28204 * t26977;
    (t96975, t96995, t96999, t97006)
}
