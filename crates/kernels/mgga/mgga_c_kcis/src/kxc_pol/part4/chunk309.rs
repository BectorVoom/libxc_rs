//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 309/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk309<F: Float>(t1131: F, t1133: F, t1021: F, t1092: F, t1010: F, t1016: F, t1025: F, t1089: F, t1124: F, t1128: F, t300: F, t975: F, t979: F) -> (F, F, F, F) {
    let t1134 = t1131 * t1133;
    let t1135 = t1021 * t1134;
    let t1136 = t1092 * t1135;
    let t1138 = t975 * t300 - F::new(0.66725e-1) * t979 * t1010 + t1016 + F::new(0.16581944444444444444e-2) * t1025 + F::new(0.24872916666666666666e-2) * t1089 - F::new(0.24872916666666666666e-2) * t1124 - F::new(0.66327777777777777776e-2) * t1128 + F::new(0.16581944444444444444e-2) * t1136;
    (t1134, t1135, t1136, t1138)
}
