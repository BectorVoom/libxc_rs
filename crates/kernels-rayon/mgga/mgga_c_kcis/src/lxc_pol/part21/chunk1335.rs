//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1335/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1335(t3329: f64, t8060: f64, t3331: f64, t95292: f64, t95294: f64, t95296: f64, t95298: f64, t95301: f64, t95303: f64, t95305: f64, t95307: f64, t95309: f64, t95311: f64, t95313: f64, t95315: f64, t95317: f64, t95319: f64, t95322: f64, t95324: f64, t95327: f64, t95329: f64, t95331: f64, t95333: f64) -> (f64, f64) {
    let t96543 = t8060 * t3329;
    let t96545 = 2.0_f64 * t96543 * t3331;
    let t96594 = -0.91666666666666666667e0_f64 * t95292 + 0.17986111111111111111e-1_f64 * t95294 - 0.28777777777777777778e0_f64 * t95296 - 0.53958333333333333334e-1_f64 * t95298 + 0.43166666666666666667e0_f64 * t95301 + 0.33333333333333333334e0_f64 * t95303 + 0.26979166666666666667e-1_f64 * t95305 - 0.28777777777777777778e0_f64 * t95307 + 0.59953703703703703705e-2_f64 * t95309 - 0.41666666666666666666e-1_f64 * t95311 - 0.125e0_f64 * t95313 - 0.625e-1_f64 * t95315 + 0.91666666666666666667e0_f64 * t95317 + 0.9375e-1_f64 * t95319 + 0.125e0_f64 * t95322 - 0.53958333333333333334e-1_f64 * t95324 + 0.5e0_f64 * t95327 - 0.21583333333333333334e0_f64 * t95329 + 0.53958333333333333334e-1_f64 * t95331 - 0.809375e-1_f64 * t95333;
    (t96545, t96594)
}
