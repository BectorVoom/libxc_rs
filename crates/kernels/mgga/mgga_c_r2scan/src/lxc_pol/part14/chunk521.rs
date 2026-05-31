//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 521/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk521<F: Float>(t322: F, t1013: F, t833: F, t829: F, t1300: F, t2394: F, t327: F, t834: F, t330: F, t1018: F, t837: F, t2393: F) -> (F, F, F, F, F, F) {
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t2397 = t1013 * t833;
    let t2400 = t1013 * t829;
    let t2405 = -F::cast_from(0.64e0_f64) * t2394 * t327 - F::cast_from(0.128e1_f64) * t2397 * t829 - F::cast_from(0.128e1_f64) * t1300 * t2400 - F::cast_from(0.64e0_f64) * t834 * t2394;
    let t2406 = t2405 * t330;
    let t2407 = t1018 * t837;
    let t2408 = t2407 * t330;
    let t2410 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t2393);
    (t2397, t2400, t2405, t2406, t2408, t2410)
}
