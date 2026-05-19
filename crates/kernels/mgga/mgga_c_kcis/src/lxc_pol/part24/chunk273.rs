//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 273/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk273<F: Float>(t1021: F, t1134: F, t1092: F, t1010: F, t1016: F, t1025: F, t1089: F, t1124: F, t1128: F, t300: F, t975: F, t979: F) -> (F, F, F) {
    let t1135 = t1021 * t1134;
    let t1136 = t1092 * t1135;
    let t1138 = t975 * t300 - F::new(0.66725e-1) * t979 * t1010 + t1016 + F::cast_from(0.16581944444444444444e-2_f64) * t1025 + F::cast_from(0.24872916666666666666e-2_f64) * t1089 - F::cast_from(0.24872916666666666666e-2_f64) * t1124 - F::cast_from(0.66327777777777777776e-2_f64) * t1128 + F::cast_from(0.16581944444444444444e-2_f64) * t1136;
    (t1135, t1136, t1138)
}
