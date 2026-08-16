//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1075/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1075(t13480: f64, t345: f64, t13462: f64, t1154: f64, t13467: f64, t10113: f64, t10115: f64, t10131: f64, t10133: f64, t10137: f64, t10141: f64, t10144: f64, t104: f64, t13578: f64, t13584: f64, t13590: f64, t13592: f64, t13594: f64, t13596: f64, t13598: f64, t13600: f64, t13602: f64) -> f64 {
    let t13605 = t345 * t13480;
    let t13608 = t345 * t13462;
    let t13611 = t1154 * t13467;
    let t13616 = 0.39814e-1_f64 * t13578 - 0.10038333333333333333e-1_f64 * t13584 - 0.77300125e-4_f64 * t13590 + 0.35222222222222222221e-2_f64 * t13592 + 0.39210208333333333333e-4_f64 * t13594 - 0.10929333333333333333e-1_f64 * t13596 + t10113 - t10115 + 0.23911438650126355246e-1_f64 * t13598 - 0.31077233446777841256e-3_f64 * t13600 + 0.7026e-2_f64 * t104 * t13602 - 0.7026e-2_f64 * t104 * t13605 + 0.1171e-2_f64 * t104 * t13608 + 0.78066666666666666667e-3_f64 * t104 * t13611 - 0.21858666666666666666e-1_f64 * t10131 + 0.70444444444444444443e-2_f64 * t10133 + t10137 + t10141 - t10144;
    t13616
}
