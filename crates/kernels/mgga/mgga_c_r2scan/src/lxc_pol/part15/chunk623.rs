//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 623/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk623<F: Float>(t322: F, t2438: F, t3382: F, t3384: F, t3412: F, t3413: F, t3416: F, t3420: F, t352: F, t855: F, t2292: F, t255: F) -> (F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t3424 = piecewise5::<F>(t323, t3382 + t3384, t331, t3412, -F::cast_from(0.21e1_f64) * t3413 * t2438 - F::cast_from(0.105e1_f64) * t855 * t3416 * t352 - F::cast_from(0.1575e1_f64) * t3420 * t2438);
    let t3428 = t2292 * t255;
    (t3424, t3428)
}
