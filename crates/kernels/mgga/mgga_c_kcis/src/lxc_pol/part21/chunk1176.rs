//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1176/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1176<F: Float>(t3329: F, t8060: F, t3331: F, t95292: F, t95294: F, t95296: F, t95298: F, t95301: F, t95303: F, t95305: F, t95307: F, t95309: F, t95311: F, t95313: F, t95315: F, t95317: F, t95319: F, t95322: F, t95324: F, t95327: F, t95329: F, t95331: F, t95333: F) -> (F, F) {
    let t96543 = t8060 * t3329;
    let t96545 = 2.0 * t96543 * t3331;
    let t96594 = -0.91666666666666666667e0 * t95292 + 0.17986111111111111111e-1 * t95294 - 0.28777777777777777778e0 * t95296 - 0.53958333333333333334e-1 * t95298 + 0.43166666666666666667e0 * t95301 + 0.33333333333333333334e0 * t95303 + 0.26979166666666666667e-1 * t95305 - 0.28777777777777777778e0 * t95307 + 0.59953703703703703705e-2 * t95309 - 0.41666666666666666666e-1 * t95311 - 0.125e0 * t95313 - 0.625e-1 * t95315 + 0.91666666666666666667e0 * t95317 + 0.9375e-1 * t95319 + 0.125e0 * t95322 - 0.53958333333333333334e-1 * t95324 + 0.5e0 * t95327 - 0.21583333333333333334e0 * t95329 + 0.53958333333333333334e-1 * t95331 - 0.809375e-1 * t95333;
    (t96545, t96594)
}
