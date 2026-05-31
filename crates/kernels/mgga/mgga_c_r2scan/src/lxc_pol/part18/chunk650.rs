//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 650/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk650<F: Float>(t322: F, t3413: F, t3420: F, t352: F, t3644: F, t3646: F, t3674: F, t3675: F, t3678: F, t855: F, t3446: F, t3453: F, t970: F) -> (F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t3685 = piecewise5::<F>(t323, t3644 + t3646, t331, t3674, -F::cast_from(0.21e1_f64) * t3413 * t3675 - F::cast_from(0.105e1_f64) * t855 * t3678 * t352 - F::cast_from(0.1575e1_f64) * t3420 * t3675);
    let t3690 = t3446 * t3453 * t970;
    (t3685, t3690)
}
