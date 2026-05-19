//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 694/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk694<F: Float>(t4741: F, t5246: F, t5416: F, t5418: F, t5422: F, t5424: F, t5426: F, t219: F, t225: F, t234: F, t61: F, t704: F) -> (F, F, F) {
    let t5429 = F::cast_from(0.17261666666666666666e2_f64) * t5246 - F::cast_from(0.69046666666666666665e1_f64) * t5416 + F::cast_from(0.10740592592592592593e2_f64) * t5418 - F::cast_from(0.44012999999999999999e0_f64) * t5422 + F::new(0.29342e0) * t5424 - F::cast_from(0.34232333333333333333e0_f64) * t5426 - F::cast_from(0.25755333333333333333e0_f64) * t4741;
    let t5431 = t219 * t5429 * t225;
    let t5433 = F::cast_from(0.5848223622634646207e0_f64) * t234 * t5431;
    let t5434 = t61 * t704;
    (t5429, t5433, t5434)
}
