//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1239/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1239<F: Float>(t1693: F, t35237: F, t20: F, t654: F, t8792: F, t716: F, t719: F, t8831: F, t705: F, t415: F, t15892: F, t2441: F, t33017: F, t1869: F, t2785: F, t34073: F, t34122: F, t34125: F, t34192: F, t34236: F, t34278: F, t34280: F, t35212: F, t35222: F, t35225: F, t35230: F, t35234: F, t9664: F, t9922: F, t9936: F, t9940: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35238 = t1693 * t35237;
    let t35242 = t8792 * t654 * t20;
    let t35243 = t1693 * t35242;
    let t35248 = t716 * t8831 * t719;
    let t35249 = t705 * t35248;
    let t35250 = t415 * t35249;
    let t35252 = t15892 * t2441;
    let t35253 = t33017 * t35252;
    let t35254 = t1869 * t35253;
    let t35260 = 0.10416666666666666667e-1 * t9664 * t35212 - 0.55555555555555555558e-1 * t34125 * t9940 - 0.55555555555555555558e-1 * t34125 * t9922 + 0.20833333333333333334e-1 * t34122 * t9940 - 0.13265555555555555555e-1 * t35222 + 0.24872916666666666666e-2 * t35225 - 0.69444444444444444446e-2 * t34073 * t9936 + 0.69444444444444444446e-2 * t9664 * t35230 - 0.24872916666666666666e-2 * t35234 + 0.55555555555555555558e-1 * t35238 * t2785 - 0.10416666666666666667e-1 * t35243 * t2785 - 0.69444444444444444446e-2 * t34236 + 0.24320185185185185185e-1 * t35250 - 0.33163888888888888888e-2 * t35254 + 0.8041666666666666667e-2 * t34192 * t9922 + 0.33163888888888888888e-2 * t34278 - 0.88437037037037037034e-2 * t34280;
    (t35238, t35242, t35243, t35248, t35249, t35250, t35252, t35253, t35254, t35260)
}
