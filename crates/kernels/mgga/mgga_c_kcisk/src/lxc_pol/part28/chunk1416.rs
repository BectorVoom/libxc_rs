//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1416/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1416<F: Float>(t34484: F, t34594: F, t10005: F, t34573: F, t112761: F, t35453: F, t33196: F, t112858: F, t35505: F, t9740: F, t112780: F, t112872: F, t117857: F, t121124: F, t121127: F, t122435: F, t24943: F, t24948: F, t33208: F, t33219: F, t33220: F, t33297: F, t35402: F, t35506: F) -> (F, F) {
    let t122472 = t34594 * t34484;
    let t122474 = t10005 * t34484;
    let t122477 = t34573 * t34484;
    let t122494 = t112761 * t35453;
    let t122495 = t33196 * t122494;
    let t122498 = t9740 * t112858 * t35505;
    let t122503 = 0.13402777777777777778e-2 * t122472 - 0.92592592592592592593e-2 * t122474 + 0.15476481481481481481e-2 * t121124 - 0.3574074074074074074e-2 * t122477 - 0.11607361111111111111e-2 * t121127 + 0.34722222222222222222e-2 * t33297 * t35506 + 0.34722222222222222222e-2 * t33208 * t35506 + 0.34722222222222222222e-2 * t9740 * t33219 * t33220 * t24943 - 0.69444444444444444444e-2 * t9740 * t117857 * t33220 * t24948 + 0.6701388888888888889e-3 * t33196 * t122435 + 0.44675925925925925927e-3 * t122495 + 0.11574074074074074074e-2 * t122498 - 0.11574074074074074074e-2 * t112780 - 0.40208333333333333334e-2 * t112872 * t35402;
    (t122494, t122503)
}
