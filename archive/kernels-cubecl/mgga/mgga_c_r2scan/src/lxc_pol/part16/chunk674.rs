//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 674/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk674<F: Float>(t1411: F, t732: F, t1524: F, t1384: F, t4811: F, t4816: F, t234: F, t105: F, t488: F, t1559: F, t282: F, t283: F) -> (F, F, F, F, F) {
    let t5029 = t732 * t1411;
    let t5034 = t732 * t1524;
    let t5037 = t4816 * t4811 * t1384;
    let t5038 = t234 * t5037;
    let t5039 = F::cast_from(0.10389515463408878255e3_f64) * t5038;
    let t5052 = F::cast_from(1.0_f64) / t488 / t105;
    let t5084 = t1559 * t282;
    let t5086 = F::cast_from(1.0_f64) / t283 / t5084;
    (t5029, t5034, t5039, t5052, t5086)
}
