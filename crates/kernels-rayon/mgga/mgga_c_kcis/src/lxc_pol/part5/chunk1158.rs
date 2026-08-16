//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1158/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1158(t10102: f64, t10109: f64, t10113: f64, t10115: f64, t10131: f64, t10133: f64, t10137: f64, t10141: f64, t10144: f64, t13558: f64, t13564: f64, t13578: f64, t13584: f64, t13590: f64, t13592: f64, t13594: f64, t13596: f64, t13598: f64, t13600: f64) -> f64 {
    let t19534 = t13558 - t13564 + 0.23911438650126355246e-1_f64 * t10102 - 0.31077233446777841256e-3_f64 * t10109 + 0.18736e-1_f64 * t13578 - 0.52833333333333333332e-2_f64 * t13584 - 0.4705225e-4_f64 * t13590 + 0.70444444444444444443e-2_f64 * t13592 + 0.78420416666666666667e-4_f64 * t13594 - 0.21858666666666666667e-1_f64 * t13596 + t10113 - t10115 + 0.47822877300252710492e-1_f64 * t13598 - 0.62154466893555682512e-3_f64 * t13600 - 0.10929333333333333333e-1_f64 * t10131 + 0.35222222222222222222e-2_f64 * t10133 + t10137 + t10141 - t10144;
    t19534
}
