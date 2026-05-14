//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1131/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1131<F: Float>(t11418: F, t1616: F, t27607: F, t28778: F, t54162: F, t7978: F, t8225: F, t27594: F, t6140: F, t12825: F, t8221: F, t27591: F, t28727: F, t28714: F, t98225: F, t8212: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99120 = t1616 * t11418;
    let t99129 = 0.23168402777777777778e-3 * t27607 * t28778;
    let t99131 = t7978 * t54162 * t8225;
    let t99133 = t27594 * t6140;
    let t99152 = t7978 * t12825 * t8221;
    let t99154 = t28727 * t27591;
    let t99157 = 0.7722800925925925926e-4 * t28714 * t27591;
    let t99173 = 0.10317654320987654321e-2 * t98225;
    let t99175 = t54162 * t8212;
    (t99120, t99129, t99131, t99133, t99152, t99154, t99157, t99173, t99175)
}
