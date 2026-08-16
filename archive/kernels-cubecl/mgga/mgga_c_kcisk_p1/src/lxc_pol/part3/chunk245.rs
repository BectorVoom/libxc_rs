//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 245/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk245<F: Float>(t288: F, t285: F, t1033: F, t1037: F, t1040: F, t1043: F, t1046: F, t1051: F) -> (F, F, F, F) {
    let t1138 = t288 * t288;
    let t1139 = F::cast_from(1.0_f64) / t1138;
    let t1140 = t285 * t1139;
    let t1147 = F::cast_from(0.1875e0_f64) * t1033 - F::cast_from(0.1875e0_f64) * t1037 - F::cast_from(0.375e0_f64) * t1040 - F::cast_from(0.4046875e-1_f64) * t1043 + F::cast_from(0.4046875e-1_f64) * t1046 + F::cast_from(0.161875e0_f64) * t1051;
    (t1138, t1139, t1140, t1147)
}
