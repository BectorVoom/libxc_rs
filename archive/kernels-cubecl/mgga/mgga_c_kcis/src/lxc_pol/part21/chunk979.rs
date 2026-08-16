//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 979/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk979<F: Float>(t14788: F, t5076: F, t1184: F, t5086: F, t1165: F, t284: F, t5078: F, t14766: F, t14769: F, t14771: F, t14773: F, t14776: F, t14779: F, t14783: F, t14786: F) -> (F, F, F, F) {
    let t14789 = t5076 * t14788;
    let t14791 = t1184 * t5086;
    let t14793 = t1165 * t284;
    let t14794 = t14793 * t5078;
    let t14796 = -t14766 / F::cast_from(64.0_f64) + t14769 / F::cast_from(72.0_f64) - t14771 / F::cast_from(12.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14773 - F::cast_from(19.0_f64) / F::cast_from(108.0_f64) * t14776 - t14779 / F::cast_from(24.0_f64) + t14783 / F::cast_from(8.0_f64) + t14786 / F::cast_from(96.0_f64) - t14789 / F::cast_from(72.0_f64) + t14791 / F::cast_from(18.0_f64) - t14794 / F::cast_from(36.0_f64);
    (t14789, t14791, t14794, t14796)
}
