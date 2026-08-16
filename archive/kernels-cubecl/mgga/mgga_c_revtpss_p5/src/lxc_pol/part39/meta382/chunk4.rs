//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1376/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1376<F: Float>(t12297: F, t12299: F, t12301: F, t12303: F, t12610: F, t16706: F, t16708: F, t16711: F, t16713: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> F {
    let t16750 = -t12610 + F::cast_from(0.13170370370370370371e-1_f64) * t12297 + F::cast_from(0.32925925925925925927e-2_f64) * t12299 - F::cast_from(0.9877777777777777778e-2_f64) * t12301 - F::cast_from(0.4938888888888888889e-2_f64) * t12303 + F::cast_from(0.65851851851851851853e-2_f64) * t16706 + F::cast_from(0.65851851851851851854e-2_f64) * t16708 - t16711 - t16713 + F::cast_from(0.16462962962962962963e-1_f64) * t16717 - F::cast_from(0.59266666666666666668e-1_f64) * t16722 - F::cast_from(0.19755555555555555556e-1_f64) * t16727 - F::cast_from(0.9877777777777777778e-2_f64) * t16731 + F::cast_from(0.88900000000000000002e-1_f64) * t16735 + F::cast_from(0.59266666666666666668e-1_f64) * t16740 + F::cast_from(0.29633333333333333334e-1_f64) * t16744 + F::cast_from(0.14816666666666666667e-1_f64) * t16748;
    t16750
}
