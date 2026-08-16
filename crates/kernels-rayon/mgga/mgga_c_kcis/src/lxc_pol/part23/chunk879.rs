//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 879/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk879(t11671: f64, t544: f64, t16055: f64, t1102: f64, t11379: f64, t11381: f64, t11384: f64, t11640: f64, t11642: f64, t11644: f64, t16457: f64, t16461: f64, t16464: f64, t16467: f64, t16470: f64, t16474: f64, t16480: f64, t16535: f64, t16539: f64, t16543: f64, t16545: f64, t16547: f64, t16549: f64, t344: f64, t4587: f64) -> f64 {
    let t16552 = t11671 * t544;
    let t16553 = t16552 * t16055;
    let t16559 = -0.65704296666666666667e-3_f64 * t1102 * t16457 + 0.26281718666666666666e-2_f64 * t4587 * t16461 - 0.13140859333333333333e-2_f64 * t1102 * t16464 + 0.52563437333333333332e-2_f64 * t4587 * t16467 - 0.65704296666666666666e-2_f64 * t1102 * t16470 + 0.492782225e-3_f64 * t1102 * t16474 + 0.13140859333333333334e-2_f64 * t11379 - 0.8760572888888888889e-3_f64 * t11381 + 0.13140859333333333333e-2_f64 * t1102 * t16480 - 0.98556445e-3_f64 * t344 * t16535 + t11384 + 0.1478346675e-2_f64 * t344 * t16539 + t16543 - t16545 - t16547 + 0.39422578e-2_f64 * t1102 * t16549 + 0.29201909629629629629e-2_f64 * t1102 * t16553 - 0.2920190962962962963e-3_f64 * t11640 + 0.43802864444444444445e-3_f64 * t11642 + 0.73004774074074074075e-3_f64 * t11644;
    t16559
}
