//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1407/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1407<F: Float>(t1599: F, t1612: F, t18223: F, t23158: F, t23174: F, t23178: F, t23182: F, t23186: F, t23192: F, t23194: F, t23200: F, t23208: F, t23211: F, t23213: F, t6141: F, t6179: F, t6185: F) -> F {
    let t23215 = t23174 / F::cast_from(1296.0_f64) - t1599 * t23178 / F::cast_from(32.0_f64) + t1599 * t23182 / F::cast_from(48.0_f64) + t1599 * t23186 / F::cast_from(576.0_f64) - t6141 * t6179 / F::cast_from(18.0_f64) - t23192 / F::cast_from(864.0_f64) - t23194 / F::cast_from(324.0_f64) - t18223 / F::cast_from(432.0_f64) - t1599 * t23200 / F::cast_from(192.0_f64) - F::cast_from(11.0_f64) / F::cast_from(216.0_f64) * t23158 * t1612 + t6141 * t6185 / F::cast_from(36.0_f64) - t23208 / F::cast_from(576.0_f64) + t23211 / F::cast_from(288.0_f64) + t23213 / F::cast_from(108.0_f64);
    t23215
}
