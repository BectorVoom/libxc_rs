//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 715/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk715<F: Float>(t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F, t5178: F, t5183: F, t5186: F) -> F {
    let t5393 = -F::cast_from(0.9375e-1_f64) * t5084 + F::cast_from(0.71944444444444444443e-1_f64) * t5087 + F::cast_from(0.101171875e-1_f64) * t5089 - F::cast_from(0.625e-1_f64) * t5092 + F::cast_from(0.53958333333333333333e-1_f64) * t5094 - F::cast_from(0.53958333333333333333e-1_f64) * t5097 + F::cast_from(0.13489583333333333333e-1_f64) * t5100 + F::cast_from(0.9375e-1_f64) * t5166 - F::cast_from(0.101171875e-1_f64) * t5170 + F::cast_from(0.101171875e-1_f64) * t5173 - F::cast_from(0.20234375e-1_f64) * t5178 - F::cast_from(0.44965277777777777777e-2_f64) * t5183 - F::cast_from(0.16666666666666666667e0_f64) * t5186;
    t5393
}
