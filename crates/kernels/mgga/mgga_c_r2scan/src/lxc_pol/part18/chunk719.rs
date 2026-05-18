//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 719/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk719<F: Float>(t1719: F, t713: F, t695: F, t717: F, t1800: F, t632: F, t645: F, t190: F, t5686: F, t1898: F, t650: F, t1907: F, t5448: F) -> (F, F, F, F, F, F) {
    let t5798 = t713 * t1719;
    let t5801 = t717 * t695;
    let t5812 = F::new(6.0) * t632 * t645 * t1800;
    let t5815 = F::new(2.0) * t632 * t190 * t5686;
    let t5818 = F::new(0.48245938496077605201e2) * t650 * t1898 * t1800;
    let t5821 = F::new(24.0) * t1907 * t190 * t5448;
    (t5798, t5801, t5812, t5815, t5818, t5821)
}
