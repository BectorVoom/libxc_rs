//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3266/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266<F: Float>(t30: F, t48294: F, t1317: F, t22790: F, t1320: F, t13550: F, t13553: F, t18280: F, t21906: F, t2255: F, t22670: F, t22769: F, t3833: F, t47025: F, t513: F, t5549: F, t605: F, t76396: F, t85406: F, t85409: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t85928 = F::cast_from(360.0_f64) * t48294;
    let t85929 = t1317 * t22790;
    let t85930 = F::cast_from(4.0_f64) * t85929;
    let t85931 = t1320 * t22790;
    let t85932 = F::cast_from(4.0_f64) * t85931;
    let t85950 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t47025 * t22769 * t605 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21906 * t2255 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13550 * t85406 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t13553 * t85409 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5549 * t18280 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3833 * t22670 * t605 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t76396);
    (t85928, t85930, t85932, t85950)
}
