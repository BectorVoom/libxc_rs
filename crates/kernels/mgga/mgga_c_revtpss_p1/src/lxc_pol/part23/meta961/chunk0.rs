//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3245/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245<F: Float>(t30: F, t46281: F, t46286: F, t5824: F, t605: F, t580: F, t1344: F, t13687: F, t13690: F, t18280: F, t21944: F, t2255: F, t22670: F, t22769: F, t3874: F, t46310: F, t5574: F, t76396: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t85390 = F::cast_from(60.0_f64) * t46281;
    let t85391 = F::cast_from(0.5848223622634646207e0_f64) * t46286;
    let t85406 = t5824 * t605;
    let t85409 = t580 * t5824;
    let t85420 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46310 * t22769 * t605 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21944 * t2255 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13687 * t85406 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13690 * t85409 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5574 * t18280 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3874 * t22670 * t605 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t76396);
    (t85390, t85391, t85406, t85409, t85420)
}
