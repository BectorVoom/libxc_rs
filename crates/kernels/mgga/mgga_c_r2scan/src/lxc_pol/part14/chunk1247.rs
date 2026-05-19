//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1247/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1247<F: Float>(t1129: F, t1131: F, t1133: F, t1135: F, t12300: F, t1310: F, t2412: F, t3530: F, t3534: F, t3538: F, t3753: F, t41978: F, t839: F, t8440: F, t8463: F, t8465: F) -> F {
    let t42067 = -F::cast_from(0.22988522834472e3_f64) * t3530 * t2412 - F::cast_from(0.22988522834472e3_f64) * t1131 * t8465 + F::cast_from(0.12405227240928e3_f64) * t3534 * t2412 + F::cast_from(0.12405227240928e3_f64) * t1133 * t8465 - F::cast_from(0.2177652951264e2_f64) * t3538 * t2412 - F::cast_from(0.2177652951264e2_f64) * t1135 * t8465 + F::cast_from(0.734774460522e2_f64) * t1129 * t8463 - F::cast_from(0.11494261417236e3_f64) * t1131 * t8463 + F::cast_from(0.6202613620464e2_f64) * t1133 * t8463 - F::cast_from(0.1088826475632e2_f64) * t1135 * t8463 - F::cast_from(0.22988522834472e3_f64) * t1129 * t8440 + F::cast_from(0.18607840861392e3_f64) * t1131 * t8440 - F::cast_from(0.4355305902528e2_f64) * t1133 * t8440 + F::cast_from(0.734774460522e2_f64) * t12300 * t839 + F::cast_from(0.367387230261e2_f64) * t3753 * t1310 - F::new(0.64e0) * t41978;
    t42067
}
