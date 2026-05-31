//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 666/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk666<F: Float>(t9048: F, t9052: F, t9056: F, t9059: F, t9063: F, t9067: F, t9070: F, t9073: F, t9080: F, t9083: F, t9087: F, t9091: F) -> F {
    let t9290 = -F::cast_from(0.125e0_f64) * t9048 - F::cast_from(0.20234375e-1_f64) * t9052 + F::cast_from(0.91666666666666666667e0_f64) * t9056 - F::cast_from(0.33333333333333333334e0_f64) * t9059 - F::cast_from(0.101171875e-1_f64) * t9063 - F::cast_from(0.44965277777777777777e-2_f64) * t9067 - F::cast_from(0.10791666666666666667e0_f64) * t9070 + F::cast_from(0.26979166666666666666e-1_f64) * t9073 - F::cast_from(0.34173611111111111111e0_f64) * t9080 + F::cast_from(0.14388888888888888889e0_f64) * t9083 - F::cast_from(0.13489583333333333333e-1_f64) * t9087 + F::cast_from(0.1875e0_f64) * t9091;
    t9290
}
