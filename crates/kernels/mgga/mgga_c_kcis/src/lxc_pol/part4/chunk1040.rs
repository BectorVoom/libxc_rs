//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1040/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1040<F: Float>(t14110: F, t5077: F, t5076: F, t1184: F, t5086: F, t1165: F, t284: F, t5078: F, t14766: F, t14769: F, t14771: F, t14773: F, t14776: F, t14779: F, t14783: F, t14786: F) -> (F, F, F, F) {
    let t14788 = t5077 * t14110;
    let t14789 = t5076 * t14788;
    let t14791 = t1184 * t5086;
    let t14793 = t1165 * t284;
    let t14794 = t14793 * t5078;
    let t14796 = -t14766 / 64.0 + t14769 / 72.0 - t14771 / 12.0 - 2.0 / 9.0 * t14773 - 19.0 / 108.0 * t14776 - t14779 / 24.0 + t14783 / 8.0 + t14786 / 96.0 - t14789 / 72.0 + t14791 / 18.0 - t14794 / 36.0;
    (t14789, t14791, t14794, t14796)
}
