//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 612/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk612<F: Float>(t322: F, t3366: F, t819: F, t3357: F, t3359: F, t3361: F, t3364: F) -> (F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t3367 = t819 * t3366;
    let t3368 = t3367 / F::cast_from(3.0_f64);
    let t3369 = t3357 + t3359 / F::cast_from(8.0_f64) - t3361 / F::cast_from(8.0_f64) + t3364 / F::cast_from(4.0_f64) + t3368;
    let t3370 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t3369);
    (t3368, t3369, t3370)
}
