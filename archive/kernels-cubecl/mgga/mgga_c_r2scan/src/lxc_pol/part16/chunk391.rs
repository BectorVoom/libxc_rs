//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 391/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk391<F: Float>(t1678: F, t614: F, t22: F, t263: F, t124: F, t5: F, t586: F, t1400: F, t1403: F, t1405: F, t1408: F) -> (F, F, F, F, F, F) {
    let t1712 = t1678 * t614;
    let t1713 = t22 * t263;
    let t1714 = t1712 * t1713;
    let t1716 = t5 * t124;
    let t1717 = t586 * t1716;
    let t1719 = -F::cast_from(0.28769444444444444445e0_f64) * t1714 + F::cast_from(0.23015555555555555556e1_f64) * t1717 + t1400 + t1403 + t1405 + t1408;
    (t1712, t1713, t1714, t1716, t1717, t1719)
}
