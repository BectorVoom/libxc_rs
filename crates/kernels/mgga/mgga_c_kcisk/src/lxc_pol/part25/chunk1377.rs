//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1377/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1377<F: Float>(t34415: F, t9724: F, t10009: F, t112921: F, t113058: F, t113059: F, t113061: F, t113123: F, t116912: F, t117646: F, t118013: F, t1636: F, t33208: F, t33219: F, t33229: F, t33297: F, t34395: F, t34416: F, t34518: F, t34548: F, t9740: F) -> (F, F) {
    let t118275 = t9724 * t34415;
    let t118282 = -0.17361111111111111111e-2 * t112921 * t10009 + 0.69444444444444444444e-2 * t33208 * t34395 + 0.69444444444444444444e-2 * t33297 * t34395 - 0.13402777777777777778e-2 * t113123 * t118013 + t113058 + 0.17411041666666666666e-2 * t116912 + 0.34722222222222222222e-2 * t9740 * t33219 * t34518 * t1636 + 0.34722222222222222222e-2 * t34416 * t33229 - 0.34722222222222222222e-2 * t9740 * t117646 + 0.13402777777777777778e-2 * t118275 * t33229 - 0.11574074074074074074e-2 * t113059 - 0.11574074074074074074e-2 * t113061 + 0.34722222222222222222e-2 * t33297 * t34548;
    (t118275, t118282)
}
