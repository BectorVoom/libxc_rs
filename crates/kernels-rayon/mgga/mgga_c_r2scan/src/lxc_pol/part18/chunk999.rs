//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 999/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk999(t11843: f64, t11845: f64, t11866: f64, t11876: f64, t11886: f64, t11502: f64, t11506: f64, t11554: f64, t986: f64, t3276: f64, t3275: f64, t11540: f64, t3579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12192 = 0.23115257973478049502e0_f64 * t11843;
    let t12193 = 0.12805040077930161442e0_f64 * t11845;
    let t12230 = 2.0_f64 / 3.0_f64 * t11866;
    let t12235 = 2.0_f64 / 3.0_f64 * t11876;
    let t12238 = 4.0_f64 / 3.0_f64 * t11886;
    let t12381 = t11506 * t11502;
    let t12382 = 3.0_f64 / 2.0_f64 * t12381;
    let t12383 = t11554 * t986;
    let t12384 = t3276 * t12383;
    let t12385 = t3275 * t12384;
    let t12386 = 5.0_f64 / 8.0_f64 * t12385;
    let t12387 = t3579 * t11540;
    (t12192, t12193, t12230, t12235, t12238, t12382, t12383, t12384, t12386, t12387)
}
