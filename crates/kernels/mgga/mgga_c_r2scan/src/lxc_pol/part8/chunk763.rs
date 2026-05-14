//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 763/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk763<F: Float>(t12: F, t3: F, t40: F, t1737: F, t4735: F, t4738: F, t640: F, t4741: F, t5246: F, t5416: F, t5418: F) -> (F, F, F, F, F) {
    let t5420 = f64::powf(t12, -0.25e1);
    let t5421 = t5420 * t3;
    let t5422 = t5421 * t40;
    let t5424 = t1737 * t4735;
    let t5426 = t640 * t4738;
    let t5429 = 0.17261666666666666666e2 * t5246 - 0.69046666666666666665e1 * t5416 + 0.10740592592592592593e2 * t5418 - 0.44012999999999999999e0 * t5422 + 0.29342e0 * t5424 - 0.34232333333333333333e0 * t5426 - 0.25755333333333333333e0 * t4741;
    (t5421, t5422, t5424, t5426, t5429)
}
