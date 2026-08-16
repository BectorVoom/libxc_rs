//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1327/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1327<F: Float>(t1803: F, t5326: F, t12297: F, t12610: F, t16706: F, t16708: F, t16711: F, t16713: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F) {
    let t21063 = t5326 * t1803;
    let t21082 = -t12610 + F::cast_from(0.65851851851851851853e-2_f64) * t12297 + F::cast_from(0.13170370370370370371e-1_f64) * t16706 + F::cast_from(0.65851851851851851853e-2_f64) * t16708 - t16711 - t16713 + F::cast_from(0.32925925925925925927e-2_f64) * t20283 + F::cast_from(0.16462962962962962963e-1_f64) * t20295 - F::cast_from(0.59266666666666666668e-1_f64) * t20300 - F::cast_from(0.19755555555555555556e-1_f64) * t20304 - F::cast_from(0.9877777777777777778e-2_f64) * t20285 + F::cast_from(0.88900000000000000002e-1_f64) * t20308 + F::cast_from(0.59266666666666666668e-1_f64) * t20312 - F::cast_from(0.4938888888888888889e-2_f64) * t20287 - F::cast_from(0.9877777777777777778e-2_f64) * t20315 + F::cast_from(0.29633333333333333334e-1_f64) * t20320 + F::cast_from(0.14816666666666666667e-1_f64) * t20290;
    (t21063, t21082)
}
