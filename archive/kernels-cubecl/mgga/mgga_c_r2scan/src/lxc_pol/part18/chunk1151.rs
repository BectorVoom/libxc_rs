//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1151/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1151<F: Float>(t1083: F, t1085: F, t1087: F, t1089: F, t2412: F, t2958: F, t3390: F, t3394: F, t3398: F, t3402: F, t3652: F, t3656: F, t3660: F, t3664: F, t9709: F, t9711: F, t9715: F) -> F {
    let t42709 = F::cast_from(0.734774460522e2_f64) * t3390 * t2958 - F::cast_from(0.11494261417236e3_f64) * t3394 * t2958 + F::cast_from(0.6202613620464e2_f64) * t3398 * t2958 - F::cast_from(0.1088826475632e2_f64) * t3402 * t2958 + F::cast_from(0.1469548921044e3_f64) * t3652 * t2412 + F::cast_from(0.734774460522e2_f64) * t1083 * t9711 - F::cast_from(0.22988522834472e3_f64) * t3656 * t2412 - F::cast_from(0.11494261417236e3_f64) * t1085 * t9711 + F::cast_from(0.12405227240928e3_f64) * t3660 * t2412 + F::cast_from(0.6202613620464e2_f64) * t1087 * t9711 - F::cast_from(0.2177652951264e2_f64) * t3664 * t2412 - F::cast_from(0.1088826475632e2_f64) * t1089 * t9711 + F::cast_from(0.1469548921044e3_f64) * t1083 * t9709 - F::cast_from(0.22988522834472e3_f64) * t1083 * t9715 - F::cast_from(0.22988522834472e3_f64) * t1085 * t9709;
    t42709
}
