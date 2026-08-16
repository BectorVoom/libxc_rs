//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1335/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1335<F: Float>(t3329: F, t8060: F, t3331: F, t95292: F, t95294: F, t95296: F, t95298: F, t95301: F, t95303: F, t95305: F, t95307: F, t95309: F, t95311: F, t95313: F, t95315: F, t95317: F, t95319: F, t95322: F, t95324: F, t95327: F, t95329: F, t95331: F, t95333: F) -> (F, F) {
    let t96543 = t8060 * t3329;
    let t96545 = F::cast_from(2.0_f64) * t96543 * t3331;
    let t96594 = -F::cast_from(0.91666666666666666667e0_f64) * t95292 + F::cast_from(0.17986111111111111111e-1_f64) * t95294 - F::cast_from(0.28777777777777777778e0_f64) * t95296 - F::cast_from(0.53958333333333333334e-1_f64) * t95298 + F::cast_from(0.43166666666666666667e0_f64) * t95301 + F::cast_from(0.33333333333333333334e0_f64) * t95303 + F::cast_from(0.26979166666666666667e-1_f64) * t95305 - F::cast_from(0.28777777777777777778e0_f64) * t95307 + F::cast_from(0.59953703703703703705e-2_f64) * t95309 - F::cast_from(0.41666666666666666666e-1_f64) * t95311 - F::cast_from(0.125e0_f64) * t95313 - F::cast_from(0.625e-1_f64) * t95315 + F::cast_from(0.91666666666666666667e0_f64) * t95317 + F::cast_from(0.9375e-1_f64) * t95319 + F::cast_from(0.125e0_f64) * t95322 - F::cast_from(0.53958333333333333334e-1_f64) * t95324 + F::cast_from(0.5e0_f64) * t95327 - F::cast_from(0.21583333333333333334e0_f64) * t95329 + F::cast_from(0.53958333333333333334e-1_f64) * t95331 - F::cast_from(0.809375e-1_f64) * t95333;
    (t96545, t96594)
}
