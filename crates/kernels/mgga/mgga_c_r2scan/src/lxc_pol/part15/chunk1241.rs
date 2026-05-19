//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1241/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1241<F: Float>(t322: F, t40851: F, t1083: F, t1085: F, t1087: F, t1089: F, t2412: F, t3390: F, t3394: F, t3398: F, t3402: F, t8440: F, t8463: F, t8465: F) -> (F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t40893 = piecewise3::<F>(t332, F::new(0.0), t40851);
    let t40923 = -F::new(0.64e0) * t40893 + F::cast_from(0.1469548921044e3_f64) * t3390 * t2412 + F::cast_from(0.1469548921044e3_f64) * t1083 * t8465 + F::cast_from(0.734774460522e2_f64) * t1083 * t8463 - F::cast_from(0.22988522834472e3_f64) * t3394 * t2412 - F::cast_from(0.22988522834472e3_f64) * t1085 * t8465 - F::cast_from(0.11494261417236e3_f64) * t1085 * t8463 + F::cast_from(0.12405227240928e3_f64) * t3398 * t2412 + F::cast_from(0.12405227240928e3_f64) * t1087 * t8465 + F::cast_from(0.6202613620464e2_f64) * t1087 * t8463 - F::cast_from(0.2177652951264e2_f64) * t3402 * t2412 - F::cast_from(0.2177652951264e2_f64) * t1089 * t8465 - F::cast_from(0.1088826475632e2_f64) * t1089 * t8463 - F::cast_from(0.22988522834472e3_f64) * t1083 * t8440 + F::cast_from(0.18607840861392e3_f64) * t1085 * t8440;
    (t40893, t40923)
}
