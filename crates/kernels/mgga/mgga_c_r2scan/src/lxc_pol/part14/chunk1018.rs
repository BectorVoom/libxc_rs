//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1018/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1018<F: Float>(t322: F, t12240: F, t3730: F, t833: F, t3735: F, t829: F, t1013: F, t3506: F, t1120: F, t2394: F, t11220: F, t11223: F, t1300: F, t2400: F, t327: F, t3509: F, t6693: F, t834: F) -> (F, F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t12241 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t12240);
    let t12244 = t3730 * t833;
    let t12253 = t3735 * t829;
    let t12256 = t3506 * t1013;
    let t12259 = t1120 * t2394;
    let t12262 = t3730 * t829;
    let t12267 = -F::cast_from(0.64e0_f64) * t12241 * t327 - F::cast_from(0.128e1_f64) * t12244 * t829 - F::cast_from(0.128e1_f64) * t11220 * t1013 - F::cast_from(0.384e1_f64) * t11223 * t2400 - F::cast_from(0.128e1_f64) * t3509 * t2394 - F::cast_from(0.384e1_f64) * t6693 * t12253 - F::cast_from(0.128e1_f64) * t1300 * t12256 - F::cast_from(0.128e1_f64) * t1300 * t12259 - F::cast_from(0.128e1_f64) * t1300 * t12262 - F::cast_from(0.64e0_f64) * t834 * t12241;
    (t12241, t12244, t12256, t12259, t12267)
}
